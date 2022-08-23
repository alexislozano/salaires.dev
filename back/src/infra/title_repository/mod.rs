pub mod in_memory;
pub mod supabase;

use crate::domain::models::Title;
use async_trait::async_trait;

pub enum FetchAllError {
    Unknown(&'static str),
}

#[async_trait]
pub trait TitleRepository: Send + Sync {
    async fn fetch_all(&self) -> Result<Vec<Title>, FetchAllError>;
}
