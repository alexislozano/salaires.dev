pub mod in_memory;

use crate::domain::models::Salary;

pub enum FetchAllError {
    Unknown,
}

pub enum InsertError {
    Unknown,
}

pub trait SalaryRepository: Send + Sync {
    fn fetch_all(&self) -> Result<Vec<Salary>, FetchAllError>;
    fn insert(&self, salary: Salary) -> Result<(), InsertError>;
}
