pub mod hcaptcha;
pub mod stub;

use crate::domain::models::Captcha;
use async_trait::async_trait;

pub enum ValidateError {
    Unknown(&'static str),
}

#[async_trait]
pub trait CaptchaService: Send + Sync {
    async fn validate(&self, captcha: Captcha) -> Result<(), ValidateError>;
}
