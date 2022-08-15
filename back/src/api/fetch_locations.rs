use std::sync::Arc;

use crate::{
    domain::{models::Location, use_cases},
    infra::LocationRepository,
};
use axum::{http::StatusCode, Extension, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    location: String,
}

impl From<Location> for Response {
    fn from(location: Location) -> Self {
        Self {
            location: location.into(),
        }
    }
}

type Error = (StatusCode, &'static str);

pub async fn fetch_locations(
    Extension(repo): Extension<Arc<dyn LocationRepository>>,
) -> Result<Json<Vec<Response>>, Error> {
    match use_cases::fetch_locations(repo).await {
        Ok(locations) => Ok(locations
            .into_iter()
            .map(|location| location.into())
            .collect::<Vec<Response>>()
            .into()),
        Err(use_cases::fetch_locations::Error::Unknown(str)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, str))
        }
    }
}
