mod api;
mod domain;
mod repositories;

use std::sync::Arc;

use dotenv::dotenv;
use repositories::{
    SupabaseCompanyRepository, SupabaseLocationRepository, SupabaseSalaryRepository,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let salary_repo = Arc::new(SupabaseSalaryRepository::new());
    let company_repo = Arc::new(SupabaseCompanyRepository::new());
    let location_repo = Arc::new(SupabaseLocationRepository::new());

    api::serve(salary_repo, company_repo, location_repo).await;
}
