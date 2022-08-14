mod api;
mod domain;
mod repositories;

use std::sync::Arc;

use dotenv::dotenv;
use repositories::InMemorySalaryRepository;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let repo = Arc::new(InMemorySalaryRepository::new());

    api::serve(repo).await;
}
