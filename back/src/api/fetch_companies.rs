use std::sync::Arc;

use crate::{domain::use_cases, repositories::CompanyRepository};
use axum::{http::StatusCode, Extension, Json};

type Error = (StatusCode, &'static str);

pub async fn fetch_companies(
    Extension(repo): Extension<Arc<dyn CompanyRepository>>,
) -> Result<Json<Vec<String>>, Error> {
    match use_cases::fetch_companies(repo).await {
        Ok(companies) => Ok(companies
            .into_iter()
            .map(|company| company.into())
            .collect::<Vec<String>>()
            .into()),
        Err(use_cases::fetch_companies::Error::Unknown(str)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, str))
        }
    }
}
