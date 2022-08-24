use std::sync::Mutex;

use super::{FetchAllError, TitleRepository};
use crate::domain::models::Title;
use async_trait::async_trait;

pub struct InMemoryTitleRepository {
    error: bool,
    titles: Mutex<Vec<Title>>,
}

impl InMemoryTitleRepository {
    #[cfg(test)]
    pub fn new() -> Self {
        Self {
            error: false,
            titles: Mutex::new(vec![]),
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
    pub fn insert(&self, title: Title) {
        self.titles.lock().unwrap().push(title);
    }
}

#[async_trait]
impl TitleRepository for InMemoryTitleRepository {
    async fn fetch_all(&self) -> Result<Vec<Title>, FetchAllError> {
        if self.error {
            return Err(FetchAllError::Unknown("error flag is on"));
        }

        let lock = match self.titles.lock() {
            Ok(lock) => lock,
            _ => return Err(FetchAllError::Unknown("could not acquire lock")),
        };

        Ok(lock.to_vec())
    }
}
