use axum::{
    body,
    http::{header, Response, StatusCode},
    response::IntoResponse,
};

const HERO: &'static [u8] = include_bytes!("hero.png");

pub async fn hero() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/png")
        .body(body::boxed(body::Full::from(HERO)))
        .unwrap()
}
