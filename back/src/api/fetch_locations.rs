use std::sync::Arc;

use crate::{domain::use_cases, repositories::LocationRepository};
use axum::{http::StatusCode, Extension, Json};

type Error = (StatusCode, &'static str);

pub async fn fetch_locations(
    Extension(repo): Extension<Arc<dyn LocationRepository>>,
) -> Result<Json<Vec<String>>, Error> {
    match use_cases::fetch_locations(repo).await {
        Ok(locations) => Ok(locations
            .into_iter()
            .map(|location| location.into())
            .collect::<Vec<String>>()
            .into()),
        Err(use_cases::fetch_locations::Error::Unknown(str)) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, str))
        }
    }
}
