use super::{CompanyRepository, FetchAllError};
use crate::domain::models::Company;
use async_trait::async_trait;
use axum::http::HeaderMap;
use serde::{Deserialize, Serialize};
use std::env;

pub struct SupabaseCompanyRepository {
    url: String,
    key: String,
}

impl SupabaseCompanyRepository {
    pub fn new() -> Self {
        Self {
            url: env::var("SUPABASE_URL").expect("SUPABASE_URL env var"),
            key: env::var("SUPABASE_KEY").expect("SUPABASE_KEY env var"),
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let api_key = self.key.parse().unwrap();
        let authorization = format!("Bearer {}", self.key).parse().unwrap();

        headers.insert("apikey", api_key);
        headers.insert("Authorization", authorization);

        headers
    }
}

#[async_trait]
impl CompanyRepository for SupabaseCompanyRepository {
    async fn fetch_all(&self) -> Result<Vec<Company>, FetchAllError> {
        let client = reqwest::Client::new();
        let res = match client
            .get(format!("{}companies?select=*&order=company", self.url))
            .headers(self.headers())
            .send()
            .await
        {
            Ok(res) => res,
            _ => return Err(FetchAllError::Unknown("could not send request")),
        };

        let supabase_companies = match res.json::<Vec<SupabaseCompany>>().await {
            Ok(companies) => companies,
            _ => return Err(FetchAllError::Unknown("could not parse json")),
        };

        let mut companies = vec![];

        for supabase_company in supabase_companies.into_iter() {
            match supabase_company.try_into() {
                Ok(company) => companies.push(company),
                _ => return Err(FetchAllError::Unknown("could not convert to domain")),
            }
        }

        Ok(companies)
    }
}

#[derive(Deserialize, Serialize)]
pub struct SupabaseCompany {
    company: String,
}

impl TryFrom<SupabaseCompany> for Company {
    type Error = ();

    fn try_from(company: SupabaseCompany) -> Result<Self, Self::Error> {
        Ok(company.company.try_into().or(Err(()))?)
    }
}
