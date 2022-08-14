use std::sync::Mutex;

use super::{DeleteError, InsertError, TokenRepository};
use crate::domain::models::Token;
use async_trait::async_trait;

pub struct InMemoryTokenRepository {
    error: bool,
    tokens: Mutex<Vec<Token>>,
}

impl InMemoryTokenRepository {
    #[cfg(test)]
    pub fn new() -> Self {
        Self {
            error: false,
            tokens: Mutex::new(vec![]),
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
impl TokenRepository for InMemoryTokenRepository {
    async fn delete(&self, token: Token) -> Result<(), DeleteError> {
        if self.error {
            return Err(DeleteError::Unknown("error flag is on"));
        }

        let mut lock = match self.tokens.lock() {
            Ok(lock) => lock,
            _ => return Err(DeleteError::Unknown("could not acquire lock")),
        };

        let index = match lock.iter().position(|t| t == &token) {
            Some(index) => index,
            None => return Err(DeleteError::Unknown("token not found")),
        };

        lock.remove(index);
        Ok(())
    }

    async fn insert(&self, token: Token) -> Result<(), InsertError> {
        if self.error {
            return Err(InsertError::Unknown("error flag is on"));
        }

        let mut lock = match self.tokens.lock() {
            Ok(lock) => lock,
            _ => return Err(InsertError::Unknown("could not acquire lock")),
        };

        lock.push(token);

        Ok(())
    }
}
