use crate::domain::models::{Captcha, Email, Token};
use crate::infra::{
    captcha_repository::DeleteError, token_repository::InsertError, token_sender::SendError,
    CaptchaRepository, TokenRepository, TokenSender,
};
use std::sync::Arc;

pub enum Error {
    Unknown(&'static str),
}

pub async fn send_token(
    captcha_repo: Arc<dyn CaptchaRepository>,
    token_repo: Arc<dyn TokenRepository>,
    sender: Arc<dyn TokenSender>,
    captcha: Captcha,
    email: Email,
) -> Result<(), Error> {
    match captcha_repo.delete(captcha).await {
        Ok(_) => {}
        Err(DeleteError::Unknown(str)) => return Err(Error::Unknown(str)),
    }

    let token = Token::generate();

    match token_repo.insert(token.clone()).await {
        Ok(()) => {}
        Err(InsertError::Unknown(str)) => return Err(Error::Unknown(str)),
    };

    match sender.send(token, email).await {
        Ok(()) => {}
        Err(SendError::Unknown(str)) => return Err(Error::Unknown(str)),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::{InMemoryCaptchaRepository, InMemoryTokenRepository, StubTokenSender};

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_captcha_could_not_be_deleted() {
        let captcha = Captcha::test();
        let email = Email::test();
        let captcha_repo = Arc::new(InMemoryCaptchaRepository::new().with_error());
        let token_repo = Arc::new(InMemoryTokenRepository::new());
        let sender = Arc::new(StubTokenSender::new());

        let res = send_token(captcha_repo, token_repo, sender, captcha, email).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_captcha_is_not_found() {
        let captcha = Captcha::test();
        let email = Email::test();
        let captcha_repo = Arc::new(InMemoryCaptchaRepository::new());
        let token_repo = Arc::new(InMemoryTokenRepository::new());
        let sender = Arc::new(StubTokenSender::new());

        let res = send_token(captcha_repo, token_repo, sender, captcha, email).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_token_could_not_be_inserted() {
        let captcha = Captcha::test();
        let email = Email::test();
        let captcha_repo = Arc::new(InMemoryCaptchaRepository::new());
        let token_repo = Arc::new(InMemoryTokenRepository::new().with_error());
        let sender = Arc::new(StubTokenSender::new());

        let res = send_token(captcha_repo, token_repo, sender, captcha, email).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_token_could_not_be_sent() {
        let captcha = Captcha::test();
        let email = Email::test();
        let captcha_repo = Arc::new(InMemoryCaptchaRepository::new());
        let token_repo = Arc::new(InMemoryTokenRepository::new());
        let sender = Arc::new(StubTokenSender::new().with_error());

        let res = send_token(captcha_repo, token_repo, sender, captcha, email).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_ok_otherwise() {
        let captcha = Captcha::test();
        let email = Email::test();
        let captcha_repo = Arc::new(InMemoryCaptchaRepository::new());
        captcha_repo.insert(captcha.clone()).await.ok();
        let token_repo = Arc::new(InMemoryTokenRepository::new());
        let sender = Arc::new(StubTokenSender::new());

        let res = send_token(captcha_repo, token_repo, sender, captcha, email).await;

        match res {
            Ok(_) => {}
            _ => unreachable!(),
        };
    }
}
