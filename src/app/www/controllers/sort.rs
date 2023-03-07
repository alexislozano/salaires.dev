use std::{collections::HashMap, sync::Arc};

use axum::extract::{Query, State};
use maud::{html, Markup};

use crate::{
    app::www::{components::notification, I18n},
    domain::{models::Order, use_cases},
    infra::SalaryRepository,
};

use super::super::fragments::salary_table;

pub async fn get(
    State(repo): State<Arc<dyn SalaryRepository>>,
    Query(params): Query<HashMap<String, String>>,
) -> Markup {
    let order = Order::from(params);

    let salaries = match use_cases::fetch_salaries(repo, order.clone()).await {
        Ok(salaries) => salaries,
        Err(use_cases::fetch_salaries::Error::Unknown(_)) => {
            return html! { (notification::view(Some(I18n::SortError.translate()))) }
        }
    };

    salary_table::view(salaries, order)
}
