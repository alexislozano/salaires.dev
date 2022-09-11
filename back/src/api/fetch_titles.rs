use std::sync::Arc;

use crate::{
    domain::{models::Title, use_cases},
    infra::TitleRepository,
};
use axum::{http::StatusCode, Extension, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    title: String,
}

impl From<Title> for Response {
    fn from(title: Title) -> Self {
        Self {
            title: title.into(),
        }
    }
}

type Error = (StatusCode, &'static str);

pub async fn fetch_titles(
    Extension(repo): Extension<Arc<dyn TitleRepository>>,
) -> Result<Json<Vec<Response>>, Error> {
    match use_cases::fetch_titles(repo).await {
        Ok(titles) => Ok(titles
            .into_iter()
            .map(|title| title.into())
            .collect::<Vec<Response>>()
            .into()),
        Err(use_cases::fetch_titles::Error::Unknown(str)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, str))
        }
    }
}
