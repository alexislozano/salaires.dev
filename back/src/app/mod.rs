mod confirm_token;
mod fetch_companies;
mod fetch_locations;
mod fetch_salaries;
mod fetch_titles;
mod insert_salary;
mod state;

use crate::infra::{
    CaptchaService, CompanyRepository, LocationRepository, SalaryRepository, TitleRepository,
    TokenRepository, TokenSender,
};
use axum::{
    http::HeaderValue,
    routing::{get, post},
    Router,
};
use confirm_token::confirm_token;
use fetch_companies::fetch_companies;
use fetch_locations::fetch_locations;
use fetch_salaries::fetch_salaries;
use fetch_titles::fetch_titles;
use insert_salary::insert_salary;
use state::State;
use std::{env, sync::Arc};
use tower_http::cors::CorsLayer;

pub async fn serve(
    salary_repo: Arc<dyn SalaryRepository>,
    company_repo: Arc<dyn CompanyRepository>,
    location_repo: Arc<dyn LocationRepository>,
    title_repo: Arc<dyn TitleRepository>,
    captcha_service: Arc<dyn CaptchaService>,
    token_repo: Arc<dyn TokenRepository>,
    token_sender: Arc<dyn TokenSender>,
) {
    let port = env::var("PORT").expect("PORT env var");
    let url = format!("0.0.0.0:{port}");
    let origin = env::var("APP_URL")
        .expect("APP_URL env var")
        .parse::<HeaderValue>()
        .expect("APP_URL should be an url");

    let state = State::new(
        salary_repo,
        company_repo,
        location_repo,
        title_repo,
        captcha_service,
        token_repo,
        token_sender,
    );

    let app = Router::new()
        .route("/salaries", get(fetch_salaries))
        .route("/salaries", post(insert_salary))
        .route("/companies", get(fetch_companies))
        .route("/locations", get(fetch_locations))
        .route("/titles", get(fetch_titles))
        .route("/tokens", post(confirm_token))
        .with_state(state)
        .layer(CorsLayer::permissive().allow_origin(origin));

    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
