use super::{DeleteError, InsertError, TokenRepository};
use crate::domain::models::{Id, Token};
use async_trait::async_trait;
use axum::http::HeaderMap;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

pub struct SupabaseTokenRepository {
    url: String,
    key: String,
}

impl SupabaseTokenRepository {
    pub fn new() -> Self {
        Self {
            url: env::var("SUPABASE_URL").expect("SUPABASE_URL env var"),
            key: env::var("SUPABASE_KEY").expect("SUPABASE_KEY env var"),
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let api_key = self.key.parse().unwrap();
        let authorization = format!("Bearer {}", self.key).parse().unwrap();

        headers.insert("apikey", api_key);
        headers.insert("Authorization", authorization);

        headers
    }
}

#[async_trait]
impl TokenRepository for SupabaseTokenRepository {
    async fn delete(&self, token: Token) -> Result<Id, DeleteError> {
        let client = reqwest::Client::new();
        let res = match client
            .get(format!(
                "{}tokens?token=eq.{}",
                self.url,
                String::from(token.clone())
            ))
            .headers(self.headers())
            .send()
            .await
        {
            Ok(res) => res,
            _ => return Err(DeleteError::Unknown("could not send request")),
        };

        let salary_id = match res.json::<Vec<SupabaseToken>>().await {
            Ok(mut tokens) => match tokens.pop() {
                Some(token) => token.salary_id.into(),
                None => return Err(DeleteError::Unknown("could not find token")),
            },
            _ => return Err(DeleteError::Unknown("could not parse json")),
        };

        match client
            .delete(format!(
                "{}tokens?token=eq.{}",
                self.url,
                String::from(token)
            ))
            .headers(self.headers())
            .send()
            .await
        {
            Ok(_) => Ok(salary_id),
            _ => Err(DeleteError::Unknown("could not send request")),
        }
    }

    async fn insert(&self, salary_id: Id, token: Token) -> Result<(), InsertError> {
        let client = reqwest::Client::new();
        match client
            .post(format!("{}tokens", self.url))
            .headers(self.headers())
            .json(&SupabaseToken::from((salary_id, token)))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            _ => Err(InsertError::Unknown("could not send request")),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SupabaseToken {
    salary_id: Uuid,
    token: String,
}

impl From<(Id, Token)> for SupabaseToken {
    fn from((salary_id, token): (Id, Token)) -> Self {
        Self {
            salary_id: salary_id.into(),
            token: token.into(),
        }
    }
}
