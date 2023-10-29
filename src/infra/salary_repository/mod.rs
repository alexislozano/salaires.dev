pub mod in_memory;
pub mod supabase;

use crate::domain::models::{salary::Key, Id, Order, Salary};
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
    async fn fetch_all_sorted(&self, order: Order<Key>) -> Result<Vec<Salary>, FetchAllError> {
        let mut salaries = self.fetch_all().await?;
        salaries.sort_by(|a, b| Salary::compare(&a, &b, &order));
        Ok(salaries)
    }
    async fn insert(&self, salary: Salary) -> Result<(), InsertError>;
}
