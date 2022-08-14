use std::sync::Mutex;

use super::{FetchAllError, InsertError, SalaryRepository};
use crate::domain::models::Salary;

pub struct InMemorySalaryRepository {
    error: bool,
    salaries: Mutex<Vec<Salary>>,
}

impl InMemorySalaryRepository {
    pub fn new() -> Self {
        Self {
            error: false,
            salaries: Mutex::new(vec![]),
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

impl SalaryRepository for InMemorySalaryRepository {
    fn fetch_all(&self) -> Result<Vec<Salary>, FetchAllError> {
        if self.error {
            return Err(FetchAllError::Unknown);
        }

        let lock = match self.salaries.lock() {
            Ok(lock) => lock,
            _ => return Err(FetchAllError::Unknown),
        };

        Ok(lock.to_vec())
    }

    fn insert(&self, salary: Salary) -> Result<(), InsertError> {
        if self.error {
            return Err(InsertError::Unknown);
        }

        let mut lock = match self.salaries.lock() {
            Ok(lock) => lock,
            _ => return Err(InsertError::Unknown),
        };

        lock.push(salary);

        Ok(())
    }
}
