use std::{collections::HashMap, sync::Arc};

use axum::extract::{Query, State};
use maud::Markup;

use crate::{
    app::www::{i18n::I18n, pages},
    domain::{
        models::{Order, Token},
        use_cases,
    },
    infra::{SalaryRepository, TokenRepository},
};

enum TokenOrNotification {
    Token(Token),
    Notification(Option<&'static str>),
}

impl From<HashMap<String, String>> for TokenOrNotification {
    fn from(hash: HashMap<String, String>) -> Self {
        match hash.get("token") {
            Some(t) => match Token::try_from(String::from(t)) {
                Ok(t) => Self::Token(t),
                _ => Self::Notification(Some(I18n::TokenConfirmationError.translate())),
            },
            _ => Self::Notification(None),
        }
    }
}

pub async fn get(
    State(salary_repo): State<Arc<dyn SalaryRepository>>,
    State(token_repo): State<Arc<dyn TokenRepository>>,
    Query(params): Query<HashMap<String, String>>,
) -> Markup {
    let order = Order::from(params.clone());

    let salaries = match use_cases::fetch_sorted_salaries(salary_repo.clone(), order.clone()).await
    {
        Ok(salaries) => salaries,
        Err(use_cases::fetch_salaries::Error::Unknown(_)) => {
            return pages::text_only::view(I18n::SalariesFetchingError.translate())
        }
    };

    let notification = match TokenOrNotification::from(params) {
        TokenOrNotification::Token(token) => {
            match use_cases::confirm_token(token_repo, salary_repo, token).await {
                Ok(()) => Some(I18n::TokenConfirmationSuccess.translate()),
                Err(use_cases::confirm_token::Error::Unknown(_)) => {
                    Some(I18n::TokenConfirmationError.translate())
                }
            }
        }
        TokenOrNotification::Notification(notification) => notification,
    };

    pages::index::view(salaries, order, notification)
}
