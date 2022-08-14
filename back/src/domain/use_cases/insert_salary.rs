use crate::domain::models::Salary;
use crate::repositories::{salary::InsertError, SalaryRepository};
use std::sync::Arc;

pub enum Error {
    Unknown(&'static str),
}

pub async fn insert_salary(repo: Arc<dyn SalaryRepository>, salary: Salary) -> Result<(), Error> {
    match repo.insert(salary).await {
        Ok(_) => Ok(()),
        Err(InsertError::Unknown(str)) => Err(Error::Unknown(str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemorySalaryRepository;

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let salary = Salary::test();
        let repo = Arc::new(InMemorySalaryRepository::new().with_error());

        let res = insert_salary(repo, salary).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_ok_otherwise() {
        let salary = Salary::test();
        let repo = Arc::new(InMemorySalaryRepository::new());

        let res = insert_salary(repo, salary).await;

        match res {
            Ok(()) => {}
            _ => unreachable!(),
        };
    }
}
