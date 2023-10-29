use std::sync::Mutex;

use super::{ConfirmError, FetchAllError, InsertError, SalaryRepository};
use crate::domain::models::{Id, Salary};
use async_trait::async_trait;

pub struct InMemorySalaryRepository {
    error: bool,
    salaries: Mutex<Vec<Salary>>,
}

impl InMemorySalaryRepository {
    #[cfg(test)]
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

#[async_trait]
impl SalaryRepository for InMemorySalaryRepository {
    async fn confirm(&self, id: Id) -> Result<(), ConfirmError> {
        if self.error {
            return Err(ConfirmError::Unknown("error flag is on"));
        }

        let mut lock = match self.salaries.lock() {
            Ok(lock) => lock,
            _ => return Err(ConfirmError::Unknown("could not acquire lock")),
        };

        let index = match lock.iter().position(|s| s.id == id) {
            Some(index) => index,
            None => return Err(ConfirmError::Unknown("salary not found")),
        };

        lock[index].confirm();

        Ok(())
    }

    async fn fetch_all(&self) -> Result<Vec<Salary>, FetchAllError> {
        if self.error {
            return Err(FetchAllError::Unknown("error flag is on"));
        }

        let lock = match self.salaries.lock() {
            Ok(lock) => lock,
            _ => return Err(FetchAllError::Unknown("could not acquire lock")),
        };

        Ok(lock.to_vec())
    }

    async fn insert(&self, salary: Salary) -> Result<(), InsertError> {
        if self.error {
            return Err(InsertError::Unknown("error flag is on"));
        }

        let mut lock = match self.salaries.lock() {
            Ok(lock) => lock,
            _ => return Err(InsertError::Unknown("could not acquire lock")),
        };

        lock.push(salary);

        Ok(())
    }
}
