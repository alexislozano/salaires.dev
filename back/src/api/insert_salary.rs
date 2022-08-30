use std::sync::Arc;

use crate::{
    domain::{models::Captcha, models::Salary, use_cases},
    infra::{CaptchaService, SalaryRepository},
};
use axum::{http::StatusCode, Extension, Json};
use chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Request {
    company: String,
    title: Option<String>,
    location: String,
    compensation: i32,
    stock: Option<i32>,
    level: Option<String>,
    company_xp: Option<i32>,
    total_xp: Option<i32>,
    captcha: String,
}

impl TryFrom<Request> for Salary {
    type Error = ();

    fn try_from(request: Request) -> Result<Self, Self::Error> {
        Ok(Self::new(
            request.company.try_into()?,
            if let Some(raw) = request.title {
                Some(raw.try_into()?)
            } else {
                None
            },
            request.location.try_into()?,
            request.compensation.try_into()?,
            Utc::today().naive_utc().into(),
            if let Some(raw) = request.stock {
                Some(raw.try_into()?)
            } else {
                None
            },
            if let Some(raw) = request.level {
                Some(raw.try_into()?)
            } else {
                None
            },
            if let Some(raw) = request.company_xp {
                Some(raw.try_into()?)
            } else {
                None
            },
            if let Some(raw) = request.total_xp {
                Some(raw.try_into()?)
            } else {
                None
            },
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
    Extension(captcha_service): Extension<Arc<dyn CaptchaService>>,
    Extension(salary_repo): Extension<Arc<dyn SalaryRepository>>,
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

    match use_cases::insert_salary(captcha_service, salary_repo, captcha, salary).await {
        Ok(()) => Ok(().into()),
        Err(use_cases::insert_salary::Error::Unknown(str)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, str))
        }
    }
}
