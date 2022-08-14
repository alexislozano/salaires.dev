use super::{InsertError, TokenRepository};
use crate::domain::models::Token;
use async_trait::async_trait;
use axum::http::HeaderMap;
use serde::Serialize;
use std::env;

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
    async fn insert(&self, token: Token) -> Result<(), InsertError> {
        let client = reqwest::Client::new();
        match client
            .post(format!("{}tokens", self.url))
            .headers(self.headers())
            .json(&SupabaseToken::from(token))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            _ => Err(InsertError::Unknown("could not send request")),
        }
    }
}

#[derive(Serialize)]
pub struct SupabaseToken {
    token: String,
}

impl From<Token> for SupabaseToken {
    fn from(token: Token) -> Self {
        Self {
            token: token.into(),
        }
    }
}
