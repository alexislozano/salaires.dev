use std::{collections::HashMap, sync::Arc};

use axum::extract::{Query, State};
use maud::Markup;

use crate::{domain::models::Order, infra::SalaryRepository};

use super::super::{fragments::salary_table, templates::page};

pub async fn index(
    State(repo): State<Arc<dyn SalaryRepository>>,
    Query(params): Query<HashMap<String, String>>,
) -> Markup {
    let order = Order::from(params);

    page::view(salary_table::view(repo, order).await)
}
