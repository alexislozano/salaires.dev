use crate::domain::models::Token;
use crate::infra::{salary_repository, token_repository, SalaryRepository, TokenRepository};
use std::sync::Arc;

pub enum Error {
    Unknown(&'static str),
}

pub async fn confirm_token(
    token_repo: Arc<dyn TokenRepository>,
    salary_repo: Arc<dyn SalaryRepository>,
    token: Token,
) -> Result<(), Error> {
    let salary_id = match token_repo.delete(token).await {
        Ok(salary_id) => salary_id,
        Err(token_repository::DeleteError::Unknown(str)) => return Err(Error::Unknown(str)),
    };

    match salary_repo.confirm(salary_id).await {
        Ok(_) => Ok(()),
        Err(salary_repository::ConfirmError::Unknown(str)) => Err(Error::Unknown(str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::models::Salary,
        infra::{InMemorySalaryRepository, InMemoryTokenRepository},
    };

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_token_could_not_be_found() {
        let token = Token::generate();
        let salary = Salary::test();
        let salary_repo = Arc::new(InMemorySalaryRepository::new());
        salary_repo.insert(salary.clone()).await.ok();
        let token_repo = Arc::new(InMemoryTokenRepository::new());

        let res = confirm_token(token_repo, salary_repo, token).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_token_could_not_be_deleted() {
        let token = Token::generate();
        let salary = Salary::test();
        let salary_repo = Arc::new(InMemorySalaryRepository::new());
        salary_repo.insert(salary.clone()).await.ok();
        let token_repo = Arc::new(InMemoryTokenRepository::new().with_error());

        let res = confirm_token(token_repo, salary_repo, token).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_salary_could_not_be_found() {
        let token = Token::generate();
        let salary = Salary::test();
        let salary_repo = Arc::new(InMemorySalaryRepository::new());
        let token_repo = Arc::new(InMemoryTokenRepository::new());
        token_repo.insert(salary.id, token.clone()).await.ok();

        let res = confirm_token(token_repo, salary_repo, token).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_salary_could_not_be_confirmed() {
        let token = Token::generate();
        let salary = Salary::test();
        let salary_repo = Arc::new(InMemorySalaryRepository::new().with_error());
        let token_repo = Arc::new(InMemoryTokenRepository::new());
        token_repo.insert(salary.id, token.clone()).await.ok();

        let res = confirm_token(token_repo, salary_repo, token).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_ok_otherwise() {
        let token = Token::generate();
        let salary = Salary::test();
        let salary_repo = Arc::new(InMemorySalaryRepository::new());
        salary_repo.insert(salary.clone()).await.ok();
        let token_repo = Arc::new(InMemoryTokenRepository::new());
        token_repo.insert(salary.id, token.clone()).await.ok();

        let res = confirm_token(token_repo, salary_repo, token).await;

        match res {
            Ok(()) => {}
            _ => unreachable!(),
        };
    }
}
