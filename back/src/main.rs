mod api;
mod domain;
mod repositories;

use std::sync::Arc;

use dotenv::dotenv;
use repositories::SupabaseSalaryRepository;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let repo = Arc::new(SupabaseSalaryRepository::new());

    api::serve(repo).await;
}
