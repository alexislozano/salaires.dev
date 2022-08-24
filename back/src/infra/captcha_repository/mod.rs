pub mod in_memory;
pub mod supabase;

use crate::domain::models::Captcha;
use async_trait::async_trait;

pub enum DeleteError {
    Unknown(&'static str),
}

pub enum InsertError {
    Unknown(&'static str),
}

#[async_trait]
pub trait CaptchaRepository: Send + Sync {
    async fn delete(&self, captcha: Captcha) -> Result<(), DeleteError>;
    async fn insert(&self, captcha: Captcha) -> Result<(), InsertError>;
}
