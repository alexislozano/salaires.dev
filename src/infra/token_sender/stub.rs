use super::{SendError, TokenSender};
use crate::domain::models::{Email, Token};
use async_trait::async_trait;

pub struct StubTokenSender {
    error: bool,
}

impl StubTokenSender {
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
impl TokenSender for StubTokenSender {
    async fn send(&self, _token: Token, _email: Email) -> Result<(), SendError> {
        if self.error {
            Err(SendError::Unknown("error flag is on"))
        } else {
            Ok(())
        }
    }
}
