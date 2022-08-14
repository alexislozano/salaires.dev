use crate::domain::models::{Salary, Token};
use crate::infra::token_repository::DeleteError;
use crate::infra::TokenRepository;
use crate::infra::{salary_repository::InsertError, SalaryRepository};
use std::sync::Arc;

pub enum Error {
    Unknown(&'static str),
}

pub async fn insert_salary(
    salary_repo: Arc<dyn SalaryRepository>,
    token_repo: Arc<dyn TokenRepository>,
    salary: Salary,
    token: Token,
) -> Result<(), Error> {
    match token_repo.delete(token).await {
        Ok(_) => {}
        Err(DeleteError::Unknown(str)) => return Err(Error::Unknown(str)),
    };

    match salary_repo.insert(salary).await {
        Ok(_) => Ok(()),
        Err(InsertError::Unknown(str)) => Err(Error::Unknown(str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::InMemorySalaryRepository;
    use crate::infra::InMemoryTokenRepository;

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_token_could_not_be_deleted() {
        let salary = Salary::test();
        let token = Token::generate();
        let token_repo = Arc::new(InMemoryTokenRepository::new().with_error());
        let salary_repo = Arc::new(InMemorySalaryRepository::new());

        let res = insert_salary(salary_repo, token_repo, salary, token).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_salary_could_not_be_inserted() {
        let salary = Salary::test();
        let token = Token::generate();
        let token_repo = Arc::new(InMemoryTokenRepository::new());
        let salary_repo = Arc::new(InMemorySalaryRepository::new().with_error());

        let res = insert_salary(salary_repo, token_repo, salary, token).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_token_is_not_found() {
        let salary = Salary::test();
        let token = Token::generate();
        let token_repo = Arc::new(InMemoryTokenRepository::new());
        let salary_repo = Arc::new(InMemorySalaryRepository::new());

        let res = insert_salary(salary_repo, token_repo, salary, token).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_ok_otherwise() {
        let salary = Salary::test();
        let token = Token::generate();
        let token_repo = Arc::new(InMemoryTokenRepository::new());
        token_repo.insert(token.clone()).await.ok();
        let salary_repo = Arc::new(InMemorySalaryRepository::new());

        let res = insert_salary(salary_repo, token_repo, salary, token).await;

        match res {
            Ok(()) => {}
            _ => unreachable!(),
        };
    }
}
