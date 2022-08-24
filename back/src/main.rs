mod api;
mod domain;
mod infra;

use std::sync::Arc;

use dotenv::dotenv;
use infra::{
    EmailTokenSender, SupabaseCaptchaRepository, SupabaseCompanyRepository,
    SupabaseLocationRepository, SupabaseSalaryRepository, SupabaseTitleRepository,
    SupabaseTokenRepository,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let salary_repo = Arc::new(SupabaseSalaryRepository::new());
    let company_repo = Arc::new(SupabaseCompanyRepository::new());
    let location_repo = Arc::new(SupabaseLocationRepository::new());
    let title_repo = Arc::new(SupabaseTitleRepository::new());
    let token_repo = Arc::new(SupabaseTokenRepository::new());
    let token_sender = Arc::new(EmailTokenSender::new());
    let captcha_repo = Arc::new(SupabaseCaptchaRepository::new());

    api::serve(
        salary_repo,
        company_repo,
        location_repo,
        title_repo,
        token_repo,
        token_sender,
        captcha_repo,
    )
    .await;
}
