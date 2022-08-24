use std::sync::Arc;

use crate::{
    domain::{models::Challenge, use_cases},
    infra::CaptchaRepository,
};
use axum::{http::StatusCode, Extension, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    challenge: String,
}

impl From<Challenge> for Response {
    fn from(challenge: Challenge) -> Self {
        Self {
            challenge: challenge.into(),
        }
    }
}

type Error = (StatusCode, &'static str);

pub async fn compute_challenge(
    Extension(repo): Extension<Arc<dyn CaptchaRepository>>,
) -> Result<Json<Response>, Error> {
    match use_cases::compute_challenge(repo).await {
        Ok(challenge) => Ok(Response::from(challenge).into()),
        Err(use_cases::compute_challenge::Error::Unknown(str)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, str))
        }
    }
}
