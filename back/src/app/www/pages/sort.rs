use std::{collections::HashMap, sync::Arc};

use axum::extract::{Query, State};
use maud::Markup;

use crate::{domain::models::Order, infra::SalaryRepository};

use super::super::fragments::salary_table;

pub async fn sort(
    State(repo): State<Arc<dyn SalaryRepository>>,
    Query(params): Query<HashMap<String, String>>,
) -> Markup {
    let order = Order::from(params);

    salary_table::view(repo, order).await
}
