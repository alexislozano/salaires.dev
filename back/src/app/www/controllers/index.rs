use std::{collections::HashMap, sync::Arc};

use axum::extract::{Query, State};
use maud::{html, Markup};

use crate::{
    app::www::{i18n::I18n, pages},
    domain::{
        models::{Order, Token},
        use_cases,
    },
    infra::{SalaryRepository, TokenRepository},
};

enum TokenParam {
    Ok(Token),
    Err,
    NotFound,
}

impl From<HashMap<String, String>> for TokenParam {
    fn from(hash: HashMap<String, String>) -> Self {
        match hash.get("token") {
            Some(t) => match Token::try_from(String::from(t)) {
                Ok(t) => Self::Ok(t),
                _ => Self::Err,
            },
            _ => Self::NotFound,
        }
    }
}

pub async fn get(
    State(salary_repo): State<Arc<dyn SalaryRepository>>,
    State(token_repo): State<Arc<dyn TokenRepository>>,
    Query(params): Query<HashMap<String, String>>,
) -> Markup {
    let order = Order::from(params.clone());

    let notification = match TokenParam::from(params) {
        TokenParam::Ok(token) => {
            match use_cases::confirm_token(token_repo, salary_repo.clone(), token).await {
                Ok(()) => Some(I18n::TokenConfirmationSuccess.translate()),
                Err(use_cases::confirm_token::Error::Unknown(_)) => {
                    Some(I18n::TokenConfirmationError.translate())
                }
            }
        }
        TokenParam::Err => Some(I18n::TokenConfirmationError.translate()),
        TokenParam::NotFound => None,
    };

    let salaries = match use_cases::fetch_salaries(salary_repo, order.clone()).await {
        Ok(salaries) => salaries,
        Err(use_cases::fetch_salaries::Error::Unknown(_)) => return html! {},
    };

    pages::index::view(salaries, order, notification)
}
