pub mod in_memory;
pub mod supabase;

use crate::domain::models::Company;
use async_trait::async_trait;

pub enum FetchAllError {
    Unknown(&'static str),
}

#[async_trait]
pub trait CompanyRepository: Send + Sync {
    async fn fetch_all(&self) -> Result<Vec<Company>, FetchAllError>;
}
