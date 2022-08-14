use crate::domain::models::{Email, Token};
use crate::infra::{
    token_repository::InsertError, token_sender::SendError, TokenRepository, TokenSender,
};
use std::sync::Arc;

pub enum Error {
    Unknown(&'static str),
}

pub async fn send_token(
    repo: Arc<dyn TokenRepository>,
    sender: Arc<dyn TokenSender>,
    email: Email,
) -> Result<(), Error> {
    let token = Token::generate();

    match repo.insert(token.clone()).await {
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
    use crate::infra::{InMemoryTokenRepository, StubTokenSender};

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_token_could_not_be_inserted() {
        let email = Email::test();
        let repo = Arc::new(InMemoryTokenRepository::new().with_error());
        let sender = Arc::new(StubTokenSender::new());

        let res = send_token(repo, sender, email).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_the_token_could_not_be_sent() {
        let email = Email::test();
        let repo = Arc::new(InMemoryTokenRepository::new());
        let sender = Arc::new(StubTokenSender::new().with_error());

        let res = send_token(repo, sender, email).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_ok_otherwise() {
        let email = Email::test();
        let repo = Arc::new(InMemoryTokenRepository::new());
        let sender = Arc::new(StubTokenSender::new());

        let res = send_token(repo, sender, email).await;

        match res {
            Ok(()) => {}
            _ => unreachable!(),
        };
    }
}
