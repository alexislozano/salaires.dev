pub mod in_memory;
pub mod supabase;

use crate::domain::models::Token;
use async_trait::async_trait;

pub enum DeleteError {
    Unknown(&'static str),
}

pub enum InsertError {
    Unknown(&'static str),
}

#[async_trait]
pub trait TokenRepository: Send + Sync {
    async fn delete(&self, token: Token) -> Result<(), DeleteError>;
    async fn insert(&self, token: Token) -> Result<(), InsertError>;
}
