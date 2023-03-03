use std::{sync::Arc, collections::HashMap};

use crate::{
    domain::{models::{Salary, salary::Order}, use_cases},
    infra::SalaryRepository,
};
use axum::{extract::{State, Query}, http::StatusCode, Json};
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    company: String,
    title: Option<String>,
    location: String,
    compensation: i32,
    date: NaiveDate,
    level: Option<String>,
    company_xp: Option<i32>,
    total_xp: Option<i32>,
}

impl From<Salary> for Response {
    fn from(salary: Salary) -> Self {
        Self {
            company: salary.company.into(),
            title: salary.title.map(|title| title.into()),
            location: salary.location.into(),
            compensation: salary.compensation.into(),
            date: salary.date.into(),
            level: salary.level.map(|level| level.into()),
            company_xp: salary.company_xp.map(|company_xp| company_xp.into()),
            total_xp: salary.total_xp.map(|total_xp| total_xp.into()),
        }
    }
}

type Error = (StatusCode, &'static str);

pub async fn fetch_salaries(
    State(repo): State<Arc<dyn SalaryRepository>>,
    Query(params): Query<HashMap<String, String>>
) -> Result<Json<Vec<Response>>, Error> {
    let order = Order::from(params);

    match use_cases::fetch_salaries(repo, order).await {
        Ok(salaries) => Ok(salaries
            .into_iter()
            .map(|salary| salary.into())
            .collect::<Vec<Response>>()
            .into()),
        Err(use_cases::fetch_salaries::Error::Unknown(str)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, str))
        }
    }
}
