mod api;
mod state;
mod www;

use crate::infra::{
    CaptchaService, CompanyRepository, LocationRepository, SalaryRepository, TitleRepository,
    TokenRepository, TokenSender,
};
use axum::{
    http::HeaderValue,
    routing::{get, post},
    Router,
};
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
        .route("/", get(www::index))
        .route("/sort", get(www::sort))
        .route("/insert", get(www::insert))
        .route("/validate", post(www::validate))
        .route("/api/salaries", get(api::fetch_salaries))
        .route("/api/salaries", post(api::insert_salary))
        .route("/api/companies", get(api::fetch_companies))
        .route("/api/locations", get(api::fetch_locations))
        .route("/api/titles", get(api::fetch_titles))
        .route("/api/tokens", post(api::confirm_token))
        .with_state(state)
        .layer(CorsLayer::permissive().allow_origin(origin));

    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
