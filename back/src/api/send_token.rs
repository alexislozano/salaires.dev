use std::sync::Arc;

use crate::{
    domain::{models::Email, use_cases},
    infra::{TokenRepository, TokenSender},
};
use axum::{http::StatusCode, Extension, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    email: String,
}

impl TryFrom<Request> for Email {
    type Error = ();

    fn try_from(request: Request) -> Result<Self, Self::Error> {
        Ok(request.email.try_into()?)
    }
}

type Error = (StatusCode, &'static str);

pub async fn send_token(
    Extension(repo): Extension<Arc<dyn TokenRepository>>,
    Extension(sender): Extension<Arc<dyn TokenSender>>,
    Json(request): Json<Request>,
) -> Result<Json<()>, Error> {
    let email = match request.try_into() {
        Ok(email) => email,
        _ => return Err((StatusCode::BAD_REQUEST, "bad body")),
    };

    match use_cases::send_token(repo, sender, email).await {
        Ok(()) => Ok(().into()),
        Err(use_cases::send_token::Error::Unknown(str)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, str))
        }
    }
}
