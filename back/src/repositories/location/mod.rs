pub mod in_memory;
pub mod supabase;

use crate::domain::models::Location;
use async_trait::async_trait;

pub enum FetchAllError {
    Unknown(&'static str),
}

#[async_trait]
pub trait LocationRepository: Send + Sync {
    async fn fetch_all(&self) -> Result<Vec<Location>, FetchAllError>;
}
