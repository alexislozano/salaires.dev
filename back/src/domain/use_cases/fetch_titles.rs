use crate::domain::models::Title;
use crate::infra::{title_repository::FetchAllError, title_repository::TitleRepository};
use std::sync::Arc;

pub enum Error {
    Unknown(&'static str),
}

pub async fn fetch_titles(repo: Arc<dyn TitleRepository>) -> Result<Vec<Title>, Error> {
    match repo.fetch_all().await {
        Ok(titles) => Ok(titles),
        Err(FetchAllError::Unknown(str)) => Err(Error::Unknown(str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::InMemoryTitleRepository;

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryTitleRepository::new().with_error());

        let res = fetch_titles(repo).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_all_locations_otherwise() {
        let title = Title::test();
        let repo = Arc::new(InMemoryTitleRepository::new());
        repo.insert(title.clone());

        let res = fetch_titles(repo).await;

        match res {
            Ok(titles) => {
                assert_eq!(vec![title], titles);
            }
            _ => unreachable!(),
        };
    }
}
