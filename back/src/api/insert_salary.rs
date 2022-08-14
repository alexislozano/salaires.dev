use std::sync::Arc;

use crate::{
    domain::{models::Salary, use_cases},
    infra::SalaryRepository,
};
use axum::{http::StatusCode, Extension, Json};
use chrono::Utc;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    company: String,
    location: String,
    compensation: i32,
    stock: Option<i32>,
    level: Option<String>,
    company_xp: Option<i32>,
    total_xp: Option<i32>,
}

impl TryFrom<Request> for Salary {
    type Error = ();

    fn try_from(request: Request) -> Result<Self, Self::Error> {
        Ok(Self::new(
            request.company.try_into()?,
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

type Error = (StatusCode, &'static str);

pub async fn insert_salary(
    Extension(repo): Extension<Arc<dyn SalaryRepository>>,
    Json(request): Json<Request>,
) -> Result<Json<()>, Error> {
    let salary = match request.try_into() {
        Ok(salary) => salary,
        _ => return Err((StatusCode::BAD_REQUEST, "bad body")),
    };

    match use_cases::insert_salary(repo, salary).await {
        Ok(()) => Ok(().into()),
        Err(use_cases::insert_salary::Error::Unknown(str)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, str))
        }
    }
}
