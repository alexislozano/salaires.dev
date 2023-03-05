use super::{CaptchaService, ValidateError};
use crate::domain::models::Captcha;
use async_trait::async_trait;

pub struct StubCaptchaService {
    error: bool,
}

impl StubCaptchaService {
    #[cfg(test)]
    pub fn new() -> Self {
        Self { error: false }
    }

    #[cfg(test)]
    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}

#[async_trait]
impl CaptchaService for StubCaptchaService {
    async fn validate(&self, _captcha: Captcha) -> Result<(), ValidateError> {
        if self.error {
            Err(ValidateError::Unknown("error flag is on"))
        } else {
            Ok(())
        }
    }
}
