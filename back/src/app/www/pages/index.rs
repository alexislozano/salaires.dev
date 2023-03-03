use std::sync::Arc;

use axum::extract::State;
use maud::{html, Markup};

use crate::{domain::use_cases, infra::SalaryRepository};

use super::super::templates::{page, _500};

pub async fn index(State(repo): State<Arc<dyn SalaryRepository>>) -> Markup {
    let _salaries = match use_cases::fetch_salaries(repo).await {
        Ok(salaries) => salaries,
        Err(use_cases::fetch_salaries::Error::Unknown(str)) => return _500::view(str),
    };

    let main = html! {};

    page::view(main)
}
