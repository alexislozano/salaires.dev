mod fetch_companies;
mod fetch_locations;
mod fetch_salaries;
mod insert_salary;
mod send_token;

use crate::infra::{
    CompanyRepository, LocationRepository, SalaryRepository, TokenRepository, TokenSender,
};
use axum::{
    error_handling::HandleErrorLayer,
    http::{HeaderValue, StatusCode},
    routing::{get, post},
    Extension, Router,
};
use fetch_companies::fetch_companies;
use fetch_locations::fetch_locations;
use fetch_salaries::fetch_salaries;
use insert_salary::insert_salary;
use send_token::send_token;
use std::{env, sync::Arc, time::Duration};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::cors::CorsLayer;

pub async fn serve(
    salary_repo: Arc<dyn SalaryRepository>,
    company_repo: Arc<dyn CompanyRepository>,
    location_repo: Arc<dyn LocationRepository>,
    token_repo: Arc<dyn TokenRepository>,
    token_sender: Arc<dyn TokenSender>,
) {
    let port = env::var("PORT").expect("PORT env var");
    let url = format!("0.0.0.0:{port}");
    let origin = env::var("APP_URL")
        .expect("APP_URL env var")
        .parse::<HeaderValue>()
        .expect("APP_URL should be an url");

    let app = Router::new()
        .route(
            "/salaries",
            get(fetch_salaries).post(insert_salary).layer(
                ServiceBuilder::new()
                    .layer(Extension(salary_repo))
                    .layer(Extension(token_repo.clone())),
            ),
        )
        .route(
            "/companies",
            get(fetch_companies).layer(Extension(company_repo)),
        )
        .route(
            "/locations",
            get(fetch_locations).layer(Extension(location_repo)),
        )
        .route(
            "/token",
            post(send_token)
                .layer(Extension(token_repo))
                .layer(Extension(token_sender)),
        )
        .layer(CorsLayer::new().allow_origin(origin))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_err| async {
                    (StatusCode::REQUEST_TIMEOUT, "timeout")
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(1, Duration::from_secs(1))),
        );

    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
