use super::{SendError, TokenSender};
use crate::domain::models::{Email, Token};
use async_trait::async_trait;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use std::env;

pub struct EmailTokenSender {
    host: String,
    email: Email,
    password: String,
}

impl EmailTokenSender {
    pub fn new() -> Self {
        Self {
            host: env::var("SMTP_HOST").expect("SMTP_HOST env var"),
            email: env::var("SMTP_EMAIL")
                .expect("SMTP_EMAIL env var")
                .try_into()
                .expect("SMTP_EMAIL should be an email address"),
            password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD env var"),
        }
    }
}

#[async_trait]
impl TokenSender for EmailTokenSender {
    async fn send(&self, token: Token, email: Email) -> Result<(), SendError> {
        let from = format!(
            "{} <{}>",
            String::from(self.email.clone()),
            String::from(self.email.clone())
        );

        let message = match Message::builder()
            .from(from.parse().unwrap())
            .to(String::from(email).parse().unwrap())
            .subject("Votre token")
            .body(String::from(token))
        {
            Ok(message) => message,
            _ => return Err(SendError::Unknown("could not generate email")),
        };

        let creds = Credentials::new(String::from(self.email.clone()), self.password.clone());

        let mailer = match AsyncSmtpTransport::<Tokio1Executor>::relay(&self.host) {
            Ok(relay) => relay.credentials(creds).build(),
            _ => return Err(SendError::Unknown("could not find smtp host")),
        };

        match mailer.send(message).await {
            Ok(_) => Ok(()),
            _ => return Err(SendError::Unknown("could not send email")),
        }
    }
}
