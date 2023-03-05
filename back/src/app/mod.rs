mod api;
mod assets;
mod state;
mod www;

use crate::infra::{
    CaptchaService, CompanyRepository, LocationRepository, SalaryRepository, TitleRepository,
    TokenRepository, TokenSender,
};
use axum::{
    routing::{delete, get, post},
    Router,
};
use state::State;
use std::{env, sync::Arc};

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
    let maintenance = env::var("MAINTENANCE")
        .expect("MAINTENANCE env var")
        .parse::<bool>()
        .expect("MAINTENANCE should be a bool");
    let no_insert = env::var("NO_INSERT")
        .expect("NO_INSERT env var")
        .parse::<bool>()
        .expect("NO_INSERT should be a bool");

    let state = State::new(
        salary_repo,
        company_repo,
        location_repo,
        title_repo,
        captcha_service,
        token_repo,
        token_sender,
    );

    let app = if maintenance {
        Router::new()
            .route("/assets/hero.png", get(assets::hero))
            .fallback(www::maintenance::get)
            .with_state(state)
    } else if no_insert {
        Router::new()
            .route("/", get(www::index::get))
            .route("/sort", get(www::sort::get))
            .route("/insert", get(www::no_insert::get))
            .route("/notification", delete(www::notification::delete))
            .route("/api/salaries", get(api::fetch_salaries))
            .route("/api/companies", get(api::fetch_companies))
            .route("/api/locations", get(api::fetch_locations))
            .route("/api/titles", get(api::fetch_titles))
            .route("/assets/hero.png", get(assets::hero))
            .fallback(www::not_found::get)
            .with_state(state)
    } else {
        Router::new()
            .route("/", get(www::index::get))
            .route("/sort", get(www::sort::get))
            .route("/insert", get(www::insert::get))
            .route("/insert", post(www::insert::post))
            .route("/validate", post(www::validate::post))
            .route("/notification", delete(www::notification::delete))
            .route("/api/salaries", get(api::fetch_salaries))
            .route("/api/salaries", post(api::insert_salary))
            .route("/api/companies", get(api::fetch_companies))
            .route("/api/locations", get(api::fetch_locations))
            .route("/api/titles", get(api::fetch_titles))
            .route("/api/tokens", post(api::confirm_token))
            .route("/assets/hero.png", get(assets::hero))
            .fallback(www::not_found::get)
            .with_state(state)
    };

    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
