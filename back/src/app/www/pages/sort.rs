use std::{collections::HashMap, sync::Arc};

use axum::extract::{Query, State};
use maud::Markup;

use crate::{
    app::www::templates::_500,
    domain::{models::Order, use_cases},
    infra::SalaryRepository,
};

use super::super::fragments::salary_table;

pub async fn sort(
    State(repo): State<Arc<dyn SalaryRepository>>,
    Query(params): Query<HashMap<String, String>>,
) -> Markup {
    let order = Order::from(params);

    let salaries = match use_cases::fetch_salaries(repo, order.clone()).await {
        Ok(salaries) => salaries,
        Err(use_cases::fetch_salaries::Error::Unknown(str)) => return _500::view(str),
    };

    salary_table::view(salaries, order)
}
