use super::{FetchAllError, TitleRepository};
use crate::domain::models::Title;
use async_trait::async_trait;
use axum::http::HeaderMap;
use serde::{Deserialize, Serialize};
use std::env;

pub struct SupabaseTitleRepository {
    url: String,
    key: String,
}

impl SupabaseTitleRepository {
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
impl TitleRepository for SupabaseTitleRepository {
    async fn fetch_all(&self) -> Result<Vec<Title>, FetchAllError> {
        let client = reqwest::Client::new();
        let res = match client
            .get(format!("{}titles?select=*&order=title", self.url))
            .headers(self.headers())
            .send()
            .await
        {
            Ok(res) => res,
            _ => return Err(FetchAllError::Unknown("could not send request")),
        };

        let supabase_titles = match res.json::<Vec<SupabaseTitle>>().await {
            Ok(titles) => titles,
            _ => return Err(FetchAllError::Unknown("could not parse json")),
        };

        let mut titles = vec![];

        for supabase_title in supabase_titles.into_iter() {
            match supabase_title.try_into() {
                Ok(title) => titles.push(title),
                _ => return Err(FetchAllError::Unknown("could not convert to domain")),
            }
        }

        Ok(titles)
    }
}

#[derive(Deserialize, Serialize)]
pub struct SupabaseTitle {
    title: String,
}

impl TryFrom<SupabaseTitle> for Title {
    type Error = ();

    fn try_from(title: SupabaseTitle) -> Result<Self, Self::Error> {
        Ok(title.title.try_into().or(Err(()))?)
    }
}
