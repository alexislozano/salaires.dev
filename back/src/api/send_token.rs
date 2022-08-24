use std::sync::Arc;

use crate::{
    domain::{models::Captcha, models::Email, use_cases},
    infra::{CaptchaRepository, TokenRepository, TokenSender},
};
use axum::{http::StatusCode, Extension, Json};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Request {
    email: String,
    captcha: String,
}

impl TryFrom<Request> for Email {
    type Error = ();

    fn try_from(request: Request) -> Result<Self, Self::Error> {
        Ok(request.email.try_into()?)
    }
}

impl TryFrom<Request> for Captcha {
    type Error = ();

    fn try_from(request: Request) -> Result<Self, Self::Error> {
        Ok(request.captcha.try_into()?)
    }
}

type Error = (StatusCode, &'static str);

pub async fn send_token(
    Extension(captcha_repo): Extension<Arc<dyn CaptchaRepository>>,
    Extension(token_repo): Extension<Arc<dyn TokenRepository>>,
    Extension(sender): Extension<Arc<dyn TokenSender>>,
    Json(request): Json<Request>,
) -> Result<Json<()>, Error> {
    let email = match request.clone().try_into() {
        Ok(email) => email,
        _ => return Err((StatusCode::BAD_REQUEST, "bad body")),
    };

    let captcha = match request.try_into() {
        Ok(captcha) => captcha,
        _ => return Err((StatusCode::BAD_REQUEST, "bad body")),
    };

    match use_cases::send_token(captcha_repo, token_repo, sender, captcha, email).await {
        Ok(()) => Ok(().into()),
        Err(use_cases::send_token::Error::Unknown(str)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, str))
        }
    }
}
