use std::sync::Arc;

use crate::{
    domain::{models::Company, use_cases},
    infra::CompanyRepository,
};
use axum::{http::StatusCode, Extension, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    company: String,
}

impl From<Company> for Response {
    fn from(company: Company) -> Self {
        Self {
            company: company.into(),
        }
    }
}

type Error = (StatusCode, &'static str);

pub async fn fetch_companies(
    Extension(repo): Extension<Arc<dyn CompanyRepository>>,
) -> Result<Json<Vec<Response>>, Error> {
    match use_cases::fetch_companies(repo).await {
        Ok(companies) => Ok(companies
            .into_iter()
            .map(|company| company.into())
            .collect::<Vec<Response>>()
            .into()),
        Err(use_cases::fetch_companies::Error::Unknown(str)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, str))
        }
    }
}
