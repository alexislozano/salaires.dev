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
    http::HeaderValue,
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
use std::{env, sync::Arc};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

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

    let app = Router::new()
        .route(
            "/salaries",
            get(fetch_salaries).layer(Extension(salary_repo.clone())),
        )
        .route(
            "/salaries",
            post(insert_salary).layer(
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
        .route("/titles", get(fetch_titles).layer(Extension(title_repo)))
        .route(
            "/tokens",
            post(send_token).layer(
                ServiceBuilder::new()
                    .layer(Extension(captcha_repo.clone()))
                    .layer(Extension(token_repo))
                    .layer(Extension(token_sender)),
            ),
        )
        .route(
            "/challenge",
            get(compute_challenge).layer(Extension(captcha_repo)),
        )
        .layer(CorsLayer::permissive().allow_origin(origin));

    axum::Server::bind(&url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
