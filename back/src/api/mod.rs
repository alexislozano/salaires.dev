mod compute_challenge;
mod fetch_companies;
mod fetch_locations;
mod fetch_salaries;
mod fetch_titles;
mod insert_salary;
mod send_token;

use crate::infra::{
    CaptchaRepository, CompanyRepository, LocationRepository, SalaryRepository, TitleRepository,
    TokenRepository, TokenSender,
};
use axum::{
    error_handling::HandleErrorLayer,
    http::{HeaderValue, StatusCode},
    routing::{get, post},
    Extension, Router,
};
use compute_challenge::compute_challenge;
use fetch_companies::fetch_companies;
use fetch_locations::fetch_locations;
use fetch_salaries::fetch_salaries;
use fetch_titles::fetch_titles;
use insert_salary::insert_salary;
use send_token::send_token;
use std::{env, sync::Arc, time::Duration};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::cors::CorsLayer;

const TIMEOUT: (reqwest::StatusCode, &str) = (StatusCode::REQUEST_TIMEOUT, "timeout");

pub async fn serve(
    salary_repo: Arc<dyn SalaryRepository>,
    company_repo: Arc<dyn CompanyRepository>,
    location_repo: Arc<dyn LocationRepository>,
    title_repo: Arc<dyn TitleRepository>,
    token_repo: Arc<dyn TokenRepository>,
    token_sender: Arc<dyn TokenSender>,
    captcha_repo: Arc<dyn CaptchaRepository>,
) {
    let port = env::var("PORT").expect("PORT env var");
    let url = format!("0.0.0.0:{port}");
    let origin = env::var("APP_URL")
        .expect("APP_URL env var")
        .parse::<HeaderValue>()
        .expect("APP_URL should be an url");
    let salary_rl = 10;
    let default_rl = 1;

    let app = Router::new()
        .route(
            "/salaries",
            get(fetch_salaries).layer(
                ServiceBuilder::new()
                    .layer(Extension(salary_repo.clone()))
                    .layer(HandleErrorLayer::new(|_| async { TIMEOUT }))
                    .layer(BufferLayer::new(1024))
                    .layer(RateLimitLayer::new(1, Duration::from_secs(default_rl))),
            ),
        )
        .route(
            "/salaries",
            post(insert_salary).layer(
                ServiceBuilder::new()
                    .layer(Extension(salary_repo))
                    .layer(Extension(token_repo.clone()))
                    .layer(HandleErrorLayer::new(|_| async { TIMEOUT }))
                    .layer(BufferLayer::new(1024))
                    .layer(RateLimitLayer::new(1, Duration::from_secs(salary_rl))),
            ),
        )
        .route(
            "/companies",
            get(fetch_companies).layer(
                ServiceBuilder::new()
                    .layer(Extension(company_repo))
                    .layer(HandleErrorLayer::new(|_| async { TIMEOUT }))
                    .layer(BufferLayer::new(1024))
                    .layer(RateLimitLayer::new(1, Duration::from_secs(default_rl))),
            ),
        )
        .route(
            "/locations",
            get(fetch_locations).layer(
                ServiceBuilder::new()
                    .layer(Extension(location_repo))
                    .layer(HandleErrorLayer::new(|_| async { TIMEOUT }))
                    .layer(BufferLayer::new(1024))
                    .layer(RateLimitLayer::new(1, Duration::from_secs(default_rl))),
            ),
        )
        .route(
            "/titles",
            get(fetch_titles).layer(
                ServiceBuilder::new()
                    .layer(Extension(title_repo))
                    .layer(HandleErrorLayer::new(|_| async { TIMEOUT }))
                    .layer(BufferLayer::new(1024))
                    .layer(RateLimitLayer::new(1, Duration::from_secs(default_rl))),
            ),
        )
        .route(
            "/tokens",
            post(send_token).layer(
                ServiceBuilder::new()
                    .layer(Extension(captcha_repo.clone()))
                    .layer(Extension(token_repo))
                    .layer(Extension(token_sender))
                    .layer(HandleErrorLayer::new(|_| async { TIMEOUT }))
                    .layer(BufferLayer::new(1024))
                    .layer(RateLimitLayer::new(1, Duration::from_secs(default_rl))),
            ),
        )
        .route(
            "/challenge",
            get(compute_challenge).layer(
                ServiceBuilder::new()
                    .layer(Extension(captcha_repo))
                    .layer(HandleErrorLayer::new(|_| async { TIMEOUT }))
                    .layer(BufferLayer::new(1024))
                    .layer(RateLimitLayer::new(1, Duration::from_secs(default_rl))),
            ),
        )
        .layer(CorsLayer::permissive().allow_origin(origin));

    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
