use super::{CaptchaService, ValidateError};
use crate::domain::models::Captcha;
use async_trait::async_trait;
use serde::Deserialize;
use std::{collections::HashMap, env};

pub struct HCaptchaService {
    secret: String,
}

impl HCaptchaService {
    pub fn new() -> Self {
        Self {
            secret: env::var("HCAPTCHA_SECRET").expect("HCAPTCHA_SECRET env var"),
        }
    }

    fn form(&self, captcha: Captcha) -> HashMap<&'static str, String> {
        let mut form = HashMap::new();

        form.insert("response", captcha.into());
        form.insert("secret", self.secret.clone());

        form
    }
}

#[async_trait]
impl CaptchaService for HCaptchaService {
    async fn validate(&self, captcha: Captcha) -> Result<(), ValidateError> {
        let client = reqwest::Client::new();
        let res = match client
            .post("https://hcaptcha.com/siteverify")
            .form(&self.form(captcha))
            .send()
            .await
        {
            Ok(res) => res,
            _ => return Err(ValidateError::Unknown("could not send request")),
        };

        let hcaptcha_response = match res.json::<HCaptchaResponse>().await {
            Ok(res) => res,
            _ => return Err(ValidateError::Unknown("could not convert to domain")),
        };

        if hcaptcha_response.success {
            Ok(())
        } else {
            Err(ValidateError::Unknown("the captcha is invalid"))
        }
    }
}

#[derive(Deserialize)]
pub struct HCaptchaResponse {
    success: bool,
}
