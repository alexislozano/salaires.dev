use std::{collections::HashMap, sync::Arc};

use axum::extract::{Query, State};
use maud::{html, Markup};

use crate::{
    app::www::pages,
    domain::{models::Order, use_cases},
    infra::SalaryRepository,
};

pub async fn get(
    State(repo): State<Arc<dyn SalaryRepository>>,
    Query(params): Query<HashMap<String, String>>,
) -> Markup {
    let order = Order::from(params);

    let salaries = match use_cases::fetch_salaries(repo, order.clone()).await {
        Ok(salaries) => salaries,
        Err(use_cases::fetch_salaries::Error::Unknown(_)) => return html! {},
    };

    pages::index::view(salaries, order)
}
