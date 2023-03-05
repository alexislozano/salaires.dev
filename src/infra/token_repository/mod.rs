pub mod in_memory;
pub mod supabase;

use crate::domain::models::{Id, Token};
use async_trait::async_trait;

pub enum DeleteError {
    Unknown(&'static str),
}

pub enum InsertError {
    Unknown(&'static str),
}

#[async_trait]
pub trait TokenRepository: Send + Sync {
    async fn delete(&self, token: Token) -> Result<Id, DeleteError>;
    async fn insert(&self, salary_id: Id, token: Token) -> Result<(), InsertError>;
}
