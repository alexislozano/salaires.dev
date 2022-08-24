use super::{CaptchaRepository, DeleteError, InsertError};
use crate::domain::models::Captcha;
use async_trait::async_trait;
use axum::http::HeaderMap;
use serde::{Deserialize, Serialize};
use std::env;

pub struct SupabaseCaptchaRepository {
    url: String,
    key: String,
}

impl SupabaseCaptchaRepository {
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
impl CaptchaRepository for SupabaseCaptchaRepository {
    async fn delete(&self, captcha: Captcha) -> Result<(), DeleteError> {
        let client = reqwest::Client::new();
        let res = match client
            .get(format!(
                "{}captchas?captcha=eq.{}",
                self.url,
                String::from(captcha.clone())
            ))
            .headers(self.headers())
            .send()
            .await
        {
            Ok(res) => res,
            _ => return Err(DeleteError::Unknown("could not send request")),
        };

        match res.json::<Vec<SupabaseCaptcha>>().await {
            Ok(captchas) => {
                if captchas.len() == 0 {
                    return Err(DeleteError::Unknown("could not find captcha"));
                }
            }
            _ => return Err(DeleteError::Unknown("could not parse json")),
        };

        match client
            .delete(format!(
                "{}captchas?captcha=eq.{}",
                self.url,
                String::from(captcha)
            ))
            .headers(self.headers())
            .send()
            .await
        {
            Ok(_) => Ok(()),
            _ => Err(DeleteError::Unknown("could not send request")),
        }
    }

    async fn insert(&self, captcha: Captcha) -> Result<(), InsertError> {
        let client = reqwest::Client::new();
        match client
            .post(format!("{}captchas", self.url))
            .headers(self.headers())
            .json(&SupabaseCaptcha::from(captcha))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            _ => Err(InsertError::Unknown("could not send request")),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SupabaseCaptcha {
    captcha: String,
}

impl From<Captcha> for SupabaseCaptcha {
    fn from(captcha: Captcha) -> Self {
        Self {
            captcha: captcha.into(),
        }
    }
}
