use crate::domain::models::Location;
use crate::repositories::{location::FetchAllError, LocationRepository};
use std::sync::Arc;

pub enum Error {
    Unknown(&'static str),
}

pub async fn fetch_locations(repo: Arc<dyn LocationRepository>) -> Result<Vec<Location>, Error> {
    match repo.fetch_all().await {
        Ok(locations) => Ok(locations),
        Err(FetchAllError::Unknown(str)) => Err(Error::Unknown(str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryLocationRepository;

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryLocationRepository::new().with_error());

        let res = fetch_locations(repo).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_all_locations_otherwise() {
        let location = Location::test();
        let repo = Arc::new(InMemoryLocationRepository::new());
        repo.insert(location.clone());

        let res = fetch_locations(repo).await;

        match res {
            Ok(locations) => {
                assert_eq!(vec![location], locations);
            }
            _ => unreachable!(),
        };
    }
}
