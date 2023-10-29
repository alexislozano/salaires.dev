use std::sync::Arc;

use axum::extract::{Query, State};
use maud::{html, Markup};
use serde::Deserialize;

use crate::{
    app::www::{components::notification, pages, I18n},
    domain::use_cases,
    infra::SalaryRepository,
};

#[derive(Deserialize, Debug)]
pub struct GraphQueryParams {
    min_size: Option<usize>,
}

pub async fn get(
    State(repo): State<Arc<dyn SalaryRepository>>,
    Query(params): Query<GraphQueryParams>,
) -> Markup {
    let salaries = match use_cases::fetch_salaries(repo).await {
        Ok(salaries) => salaries,
        Err(use_cases::fetch_salaries::Error::Unknown(_)) => {
            return html! { (notification::view(Some(I18n::SortError.translate()))) };
        }
    };

    pages::graphs::view(salaries, params.min_size.unwrap_or(0), None)
}
