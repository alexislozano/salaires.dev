use std::sync::Arc;

use crate::{
    domain::{models::Captcha, models::Id, models::Salary, models::Status, use_cases},
    infra::{CaptchaService, SalaryRepository, TokenRepository, TokenSender},
};
use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Request {
    email: String,
    company: String,
    title: Option<String>,
    location: String,
    compensation: i32,
    level: Option<String>,
    company_xp: Option<i32>,
    total_xp: Option<i32>,
    captcha: String,
}

impl TryFrom<Request> for Salary {
    type Error = ();

    fn try_from(request: Request) -> Result<Self, Self::Error> {
        Ok(Self::new(
            Id::generate(),
            request.email.try_into().or(Err(()))?,
            request.company.try_into().or(Err(()))?,
            if let Some(raw) = request.title {
                Some(raw.try_into().or(Err(()))?)
            } else {
                None
            },
            request.location.try_into().or(Err(()))?,
            request.compensation.try_into().or(Err(()))?,
            Utc::now().date_naive().into(),
            if let Some(raw) = request.level {
                Some(raw.try_into().or(Err(()))?)
            } else {
                None
            },
            if let Some(raw) = request.company_xp {
                Some(raw.try_into().or(Err(()))?)
            } else {
                None
            },
            if let Some(raw) = request.total_xp {
                Some(raw.try_into().or(Err(()))?)
            } else {
                None
            },
            Status::Waiting,
        ))
    }
}

impl TryFrom<Request> for Captcha {
    type Error = ();

    fn try_from(request: Request) -> Result<Self, Self::Error> {
        Ok(request.captcha.try_into()?)
    }
}

type Error = (StatusCode, &'static str);

pub async fn insert_salary(
    State(captcha_service): State<Arc<dyn CaptchaService>>,
    State(salary_repo): State<Arc<dyn SalaryRepository>>,
    State(token_repo): State<Arc<dyn TokenRepository>>,
    State(token_sender): State<Arc<dyn TokenSender>>,
    Json(request): Json<Request>,
) -> Result<Json<()>, Error> {
    let salary = match request.clone().try_into() {
        Ok(salary) => salary,
        _ => return Err((StatusCode::BAD_REQUEST, "bad body")),
    };

    let captcha = match request.try_into() {
        Ok(captcha) => captcha,
        _ => return Err((StatusCode::BAD_REQUEST, "bad body")),
    };

    match use_cases::insert_salary(
        captcha_service,
        salary_repo,
        token_repo,
        token_sender,
        captcha,
        salary,
    )
    .await
    {
        Ok(()) => Ok(().into()),
        Err(use_cases::insert_salary::Error::Unknown(str)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, str))
        }
    }
}
