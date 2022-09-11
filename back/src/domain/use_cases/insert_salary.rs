use crate::domain::models::{Captcha, Salary, Token};
use crate::infra::{
    captcha_service, salary_repository, token_repository, token_sender, CaptchaService,
    SalaryRepository, TokenRepository, TokenSender,
};
use std::sync::Arc;

pub enum Error {
    Unknown(&'static str),
}

pub async fn insert_salary(
    service: Arc<dyn CaptchaService>,
    salary_repo: Arc<dyn SalaryRepository>,
    token_repo: Arc<dyn TokenRepository>,
    token_sender: Arc<dyn TokenSender>,
    captcha: Captcha,
    salary: Salary,
) -> Result<(), Error> {
    match service.validate(captcha).await {
        Ok(_) => {}
        Err(captcha_service::ValidateError::Unknown(str)) => return Err(Error::Unknown(str)),
    };

    match salary_repo.insert(salary.clone()).await {
        Ok(_) => {}
        Err(salary_repository::InsertError::Unknown(str)) => return Err(Error::Unknown(str)),
    };

    let token = Token::generate();

    match token_sender.send(token.clone(), salary.email.clone()).await {
        Ok(_) => {}
        Err(token_sender::SendError::Unknown(str)) => return Err(Error::Unknown(str)),
    };

    match token_repo.insert(salary.id.clone(), token).await {
        Ok(_) => Ok(()),
        Err(token_repository::InsertError::Unknown(str)) => Err(Error::Unknown(str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::{
        InMemorySalaryRepository, InMemoryTokenRepository, StubCaptchaService, StubTokenSender,
    };

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_captcha_is_invalid() {
        let captcha = Captcha::test();
        let salary = Salary::test();
        let service = Arc::new(StubCaptchaService::new().with_error());
        let salary_repo = Arc::new(InMemorySalaryRepository::new());
        let token_repo = Arc::new(InMemoryTokenRepository::new());
        let token_sender = Arc::new(StubTokenSender::new());

        let res = insert_salary(
            service,
            salary_repo,
            token_repo,
            token_sender,
            captcha,
            salary,
        )
        .await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_salary_could_not_be_inserted() {
        let captcha = Captcha::test();
        let salary = Salary::test();
        let service = Arc::new(StubCaptchaService::new());
        let salary_repo = Arc::new(InMemorySalaryRepository::new().with_error());
        let token_repo = Arc::new(InMemoryTokenRepository::new());
        let token_sender = Arc::new(StubTokenSender::new());

        let res = insert_salary(
            service,
            salary_repo,
            token_repo,
            token_sender,
            captcha,
            salary,
        )
        .await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_token_could_not_be_inserted() {
        let captcha = Captcha::test();
        let salary = Salary::test();
        let service = Arc::new(StubCaptchaService::new());
        let salary_repo = Arc::new(InMemorySalaryRepository::new());
        let token_repo = Arc::new(InMemoryTokenRepository::new().with_error());
        let token_sender = Arc::new(StubTokenSender::new());

        let res = insert_salary(
            service,
            salary_repo,
            token_repo,
            token_sender,
            captcha,
            salary,
        )
        .await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_token_could_not_be_sent() {
        let captcha = Captcha::test();
        let salary = Salary::test();
        let service = Arc::new(StubCaptchaService::new());
        let salary_repo = Arc::new(InMemorySalaryRepository::new());
        let token_repo = Arc::new(InMemoryTokenRepository::new());
        let token_sender = Arc::new(StubTokenSender::new().with_error());

        let res = insert_salary(
            service,
            salary_repo,
            token_repo,
            token_sender,
            captcha,
            salary,
        )
        .await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_ok_otherwise() {
        let captcha = Captcha::test();
        let salary = Salary::test();
        let service = Arc::new(StubCaptchaService::new());
        let salary_repo = Arc::new(InMemorySalaryRepository::new());
        let token_repo = Arc::new(InMemoryTokenRepository::new());
        let token_sender = Arc::new(StubTokenSender::new());

        let res = insert_salary(
            service,
            salary_repo,
            token_repo,
            token_sender,
            captcha,
            salary,
        )
        .await;

        match res {
            Ok(()) => {}
            _ => unreachable!(),
        };
    }
}
