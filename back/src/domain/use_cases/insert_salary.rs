use crate::domain::models::{Captcha, Salary};
use crate::infra::captcha_service::ValidateError;
use crate::infra::CaptchaService;
use crate::infra::{salary_repository::InsertError, SalaryRepository};
use std::sync::Arc;

pub enum Error {
    Unknown(&'static str),
}

pub async fn insert_salary(
    service: Arc<dyn CaptchaService>,
    repo: Arc<dyn SalaryRepository>,
    captcha: Captcha,
    salary: Salary,
) -> Result<(), Error> {
    match service.validate(captcha).await {
        Ok(_) => {}
        Err(ValidateError::Unknown(str)) => return Err(Error::Unknown(str)),
    };

    match repo.insert(salary).await {
        Ok(_) => Ok(()),
        Err(InsertError::Unknown(str)) => Err(Error::Unknown(str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::{InMemorySalaryRepository, StubCaptchaService};

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_captcha_is_invalid() {
        let captcha = Captcha::test();
        let salary = Salary::test();
        let service = Arc::new(StubCaptchaService::new().with_error());
        let repo = Arc::new(InMemorySalaryRepository::new());

        let res = insert_salary(service, repo, captcha, salary).await;

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
        let repo = Arc::new(InMemorySalaryRepository::new().with_error());

        let res = insert_salary(service, repo, captcha, salary).await;

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
        let repo = Arc::new(InMemorySalaryRepository::new());

        let res = insert_salary(service, repo, captcha, salary).await;

        match res {
            Ok(()) => {}
            _ => unreachable!(),
        };
    }
}
