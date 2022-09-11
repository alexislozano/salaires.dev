pub mod in_memory;
pub mod supabase;

use crate::domain::models::{Id, Salary};
use async_trait::async_trait;

pub enum ConfirmError {
    Unknown(&'static str),
}

pub enum FetchAllError {
    Unknown(&'static str),
}

pub enum InsertError {
    Unknown(&'static str),
}

#[async_trait]
pub trait SalaryRepository: Send + Sync {
    async fn confirm(&self, id: Id) -> Result<(), ConfirmError>;
    async fn fetch_all(&self) -> Result<Vec<Salary>, FetchAllError>;
    async fn insert(&self, salary: Salary) -> Result<(), InsertError>;
}
