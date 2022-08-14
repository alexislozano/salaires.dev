use std::sync::Mutex;

use super::{CompanyRepository, FetchAllError};
use crate::domain::models::Company;
use async_trait::async_trait;

pub struct InMemoryCompanyRepository {
    error: bool,
    companies: Mutex<Vec<Company>>,
}

impl InMemoryCompanyRepository {
    #[cfg(test)]
    pub fn new() -> Self {
        Self {
            error: false,
            companies: Mutex::new(vec![]),
        }
    }

    #[cfg(test)]
    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }

    #[cfg(test)]
    pub fn insert(&self, company: Company) {
        self.companies.lock().unwrap().push(company);
    }
}

#[async_trait]
impl CompanyRepository for InMemoryCompanyRepository {
    async fn fetch_all(&self) -> Result<Vec<Company>, FetchAllError> {
        if self.error {
            return Err(FetchAllError::Unknown("error flag is on"));
        }

        let lock = match self.companies.lock() {
            Ok(lock) => lock,
            _ => return Err(FetchAllError::Unknown("could not acquire lock")),
        };

        Ok(lock.to_vec())
    }
}
