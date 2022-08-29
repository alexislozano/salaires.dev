use crate::domain::models::Salary;
use crate::infra::{salary_repository::InsertError, SalaryRepository};
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
    use crate::infra::InMemorySalaryRepository;

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_salary_could_not_be_inserted() {
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
