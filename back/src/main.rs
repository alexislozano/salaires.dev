mod api;
mod domain;
mod infra;

use std::sync::Arc;

use dotenv::dotenv;
use infra::{
    HCaptchaService, SupabaseCompanyRepository, SupabaseLocationRepository,
    SupabaseSalaryRepository, SupabaseTitleRepository,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let salary_repo = Arc::new(SupabaseSalaryRepository::new());
    let company_repo = Arc::new(SupabaseCompanyRepository::new());
    let location_repo = Arc::new(SupabaseLocationRepository::new());
    let title_repo = Arc::new(SupabaseTitleRepository::new());
    let captcha_service = Arc::new(HCaptchaService::new());

    api::serve(
        salary_repo,
        company_repo,
        location_repo,
        title_repo,
        captcha_service,
    )
    .await;
}
