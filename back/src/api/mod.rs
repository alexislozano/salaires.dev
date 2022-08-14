mod fetch_salaries;
mod insert_salary;

use crate::repositories::SalaryRepository;
use axum::{routing::get, Extension, Router};
use fetch_salaries::fetch_salaries;
use insert_salary::insert_salary;
use std::{env, sync::Arc};

pub async fn serve(repo: Arc<dyn SalaryRepository>) {
    let port = env::var("PORT").expect("PORT env var");
    let url = format!("0.0.0.0:{port}");

    let app = Router::new()
        .route("/", get(fetch_salaries).post(insert_salary))
        .layer(Extension(repo));

    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
