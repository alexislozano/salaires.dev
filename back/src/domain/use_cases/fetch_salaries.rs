use crate::domain::models::Salary;
use crate::repositories::{FetchAllError, SalaryRepository};
use std::sync::Arc;

pub enum Error {
    Unknown(&'static str),
}

pub async fn fetch_salaries(repo: Arc<dyn SalaryRepository>) -> Result<Vec<Salary>, Error> {
    match repo.fetch_all().await {
        Ok(salaries) => Ok(salaries),
        Err(FetchAllError::Unknown(str)) => Err(Error::Unknown(str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemorySalaryRepository;

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemorySalaryRepository::new().with_error());

        let res = fetch_salaries(repo).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_all_salaries_otherwise() {
        let salary = Salary::test();
        let repo = Arc::new(InMemorySalaryRepository::new());
        repo.insert(salary.clone()).await.ok();

        let res = fetch_salaries(repo).await;

        match res {
            Ok(salaries) => {
                assert_eq!(vec![salary], salaries);
            }
            _ => unreachable!(),
        };
    }
}
