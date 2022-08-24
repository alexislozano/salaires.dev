use std::sync::Mutex;

use super::{CaptchaRepository, DeleteError, InsertError};
use crate::domain::models::Captcha;
use async_trait::async_trait;

pub struct InMemoryCaptchaRepository {
    error: bool,
    captchas: Mutex<Vec<Captcha>>,
}

impl InMemoryCaptchaRepository {
    #[cfg(test)]
    pub fn new() -> Self {
        Self {
            error: false,
            captchas: Mutex::new(vec![]),
        }
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
impl CaptchaRepository for InMemoryCaptchaRepository {
    async fn delete(&self, captcha: Captcha) -> Result<(), DeleteError> {
        if self.error {
            return Err(DeleteError::Unknown("error flag is on"));
        }

        let mut lock = match self.captchas.lock() {
            Ok(lock) => lock,
            _ => return Err(DeleteError::Unknown("could not acquire lock")),
        };

        let index = match lock.iter().position(|c| c == &captcha) {
            Some(index) => index,
            None => return Err(DeleteError::Unknown("captcha not found")),
        };

        lock.remove(index);
        Ok(())
    }

    async fn insert(&self, captcha: Captcha) -> Result<(), InsertError> {
        if self.error {
            return Err(InsertError::Unknown("error flag is on"));
        }

        let mut lock = match self.captchas.lock() {
            Ok(lock) => lock,
            _ => return Err(InsertError::Unknown("could not acquire lock")),
        };

        lock.push(captcha);

        Ok(())
    }
}
