mod api;
mod domain;
mod infra;

use std::sync::Arc;

use dotenv::dotenv;
use infra::{
    EmailTokenSender, SupabaseCompanyRepository, SupabaseLocationRepository,
    SupabaseSalaryRepository, SupabaseTokenRepository,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let salary_repo = Arc::new(SupabaseSalaryRepository::new());
    let company_repo = Arc::new(SupabaseCompanyRepository::new());
    let location_repo = Arc::new(SupabaseLocationRepository::new());
    let token_repo = Arc::new(SupabaseTokenRepository::new());
    let token_sender = Arc::new(EmailTokenSender::new());

    api::serve(
        salary_repo,
        company_repo,
        location_repo,
        token_repo,
        token_sender,
    )
    .await;
}
