use std::sync::Mutex;

use super::{FetchAllError, LocationRepository};
use crate::domain::models::Location;
use async_trait::async_trait;

pub struct InMemoryLocationRepository {
    error: bool,
    locations: Mutex<Vec<Location>>,
}

impl InMemoryLocationRepository {
    #[cfg(test)]
    pub fn new() -> Self {
        Self {
            error: false,
            locations: Mutex::new(vec![]),
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
    pub fn insert(&self, location: Location) {
        self.locations.lock().unwrap().push(location);
    }
}

#[async_trait]
impl LocationRepository for InMemoryLocationRepository {
    async fn fetch_all(&self) -> Result<Vec<Location>, FetchAllError> {
        if self.error {
            return Err(FetchAllError::Unknown("error flag is on"));
        }

        let lock = match self.locations.lock() {
            Ok(lock) => lock,
            _ => return Err(FetchAllError::Unknown("could not acquire lock")),
        };

        Ok(lock.to_vec())
    }
}
