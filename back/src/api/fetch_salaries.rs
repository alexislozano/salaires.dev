use std::sync::Arc;

use crate::{
    domain::{models::Salary, use_cases},
    repositories::SalaryRepository,
};
use axum::{http::StatusCode, Extension, Json};
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    company: String,
    location: String,
    compensation: i32,
    date: NaiveDate,
    stock: Option<i32>,
    level: Option<String>,
    company_xp: Option<i32>,
    total_xp: Option<i32>,
}

impl From<Salary> for Response {
    fn from(salary: Salary) -> Self {
        Self {
            company: salary.company.into(),
            location: salary.location.into(),
            compensation: salary.compensation.into(),
            date: salary.date.into(),
            stock: salary.stock.map(|stock| stock.into()),
            level: salary.level.map(|level| level.into()),
            company_xp: salary.company_xp.map(|company_xp| company_xp.into()),
            total_xp: salary.total_xp.map(|total_xp| total_xp.into()),
        }
    }
}

pub async fn fetch_salaries(
    Extension(repo): Extension<Arc<dyn SalaryRepository>>,
) -> Result<Json<Vec<Response>>, StatusCode> {
    match use_cases::fetch_salaries(repo) {
        Ok(salaries) => Ok(salaries
            .into_iter()
            .map(|salary| Response::from(salary))
            .collect::<Vec<Response>>()
            .into()),
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
