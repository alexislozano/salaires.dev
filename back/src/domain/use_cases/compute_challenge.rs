use crate::domain::models::captcha::{self, Challenge};
use crate::infra::{captcha_repository::InsertError, CaptchaRepository};
use std::sync::Arc;

pub enum Error {
    Unknown(&'static str),
}

pub async fn compute_challenge(repo: Arc<dyn CaptchaRepository>) -> Result<Challenge, Error> {
    let (captcha, challenge) = match captcha::generate() {
        Ok((captcha, challenge)) => (captcha, challenge),
        Err(()) => return Err(Error::Unknown("could not generate captcha")),
    };

    match repo.insert(captcha).await {
        Ok(()) => {}
        Err(InsertError::Unknown(str)) => return Err(Error::Unknown(str)),
    };

    Ok(challenge)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::InMemoryCaptchaRepository;

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_captcha_could_not_be_inserted() {
        let repo = Arc::new(InMemoryCaptchaRepository::new().with_error());

        let res = compute_challenge(repo).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_ok_otherwise() {
        let repo = Arc::new(InMemoryCaptchaRepository::new());

        let res = compute_challenge(repo).await;

        match res {
            Ok(_) => {}
            _ => unreachable!(),
        };
    }
}
