use super::{CaptchaService, ValidateError};
use crate::domain::models::Captcha;
use async_trait::async_trait;
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
        match client
            .post("http://httpbin.org")
            .form(&self.form(captcha))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            _ => return Err(ValidateError::Unknown("the captcha is invalid")),
        }
    }
}
