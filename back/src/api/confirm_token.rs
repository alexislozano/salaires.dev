use std::sync::Arc;

use crate::{
    domain::{models::Token, use_cases},
    infra::{SalaryRepository, TokenRepository},
};
use axum::{http::StatusCode, Extension, Json};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Request {
    token: String,
}

impl TryFrom<Request> for Token {
    type Error = ();

    fn try_from(request: Request) -> Result<Self, Self::Error> {
        Ok(request.token.try_into()?)
    }
}

type Error = (StatusCode, &'static str);

pub async fn confirm_token(
    Extension(token_repo): Extension<Arc<dyn TokenRepository>>,
    Extension(salary_repo): Extension<Arc<dyn SalaryRepository>>,
    Json(request): Json<Request>,
) -> Result<Json<()>, Error> {
    let token = match request.try_into() {
        Ok(token) => token,
        _ => return Err((StatusCode::BAD_REQUEST, "bad body")),
    };

    match use_cases::confirm_token(token_repo, salary_repo, token).await {
        Ok(()) => Ok(().into()),
        Err(use_cases::confirm_token::Error::Unknown(str)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, str))
        }
    }
}
