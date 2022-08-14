pub mod email;
pub mod stub;

use crate::domain::models::{Email, Token};
use async_trait::async_trait;

pub enum SendError {
    Unknown(&'static str),
}

#[async_trait]
pub trait TokenSender: Send + Sync {
    async fn send(&self, token: Token, email: Email) -> Result<(), SendError>;
}
