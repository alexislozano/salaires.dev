mod fetch_companies;
mod fetch_locations;
mod fetch_salaries;
mod insert_salary;

use crate::repositories::{CompanyRepository, LocationRepository, SalaryRepository};
use axum::{routing::get, Extension, Router};
use fetch_companies::fetch_companies;
use fetch_locations::fetch_locations;
use fetch_salaries::fetch_salaries;
use insert_salary::insert_salary;
use std::{env, sync::Arc};

pub async fn serve(
    salary_repo: Arc<dyn SalaryRepository>,
    company_repo: Arc<dyn CompanyRepository>,
    location_repo: Arc<dyn LocationRepository>,
) {
    let port = env::var("PORT").expect("PORT env var");
    let url = format!("0.0.0.0:{port}");

    let app = Router::new()
        .route(
            "/salaries",
            get(fetch_salaries)
                .post(insert_salary)
                .layer(Extension(salary_repo)),
        )
        .route(
            "/companies",
            get(fetch_companies).layer(Extension(company_repo)),
        )
        .route(
            "/locations",
            get(fetch_locations).layer(Extension(location_repo)),
        );

    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
