use crate::domain::models::{salary::Key, Order, Salary};
use crate::infra::{salary_repository::FetchAllError, SalaryRepository};
use std::sync::Arc;

pub enum Error {
    Unknown(&'static str),
}

pub async fn fetch_salaries(
    repo: Arc<dyn SalaryRepository>,
    order: Order<Key>,
) -> Result<Vec<Salary>, Error> {
    match repo.fetch_all(order).await {
        Ok(salaries) => Ok(salaries),
        Err(FetchAllError::Unknown(str)) => Err(Error::Unknown(str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::{salary::Key, Direction};
    use crate::infra::InMemorySalaryRepository;

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemorySalaryRepository::new().with_error());

        let res = fetch_salaries(repo, Order::default()).await;

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

        let res = fetch_salaries(repo, Order::default()).await;

        match res {
            Ok(salaries) => {
                assert_eq!(vec![salary], salaries);
            }
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_all_salaries_in_the_asked_order() {
        let mut salary1 = Salary::test();
        let mut salary2 = Salary::test();
        salary1.company = String::from("B").try_into().unwrap();
        salary2.company = String::from("A").try_into().unwrap();
        let repo = Arc::new(InMemorySalaryRepository::new());
        repo.insert(salary1.clone()).await.ok();
        repo.insert(salary2.clone()).await.ok();
        let order = Order::new(Key::Company, Direction::Asc);

        let res = fetch_salaries(repo, order).await;

        match res {
            Ok(salaries) => {
                assert_eq!(vec![salary2, salary1], salaries);
            }
            _ => unreachable!(),
        };
    }
}
