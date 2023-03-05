use super::{FetchAllError, LocationRepository};
use crate::domain::models::Location;
use async_trait::async_trait;
use axum::http::HeaderMap;
use serde::{Deserialize, Serialize};
use std::env;

pub struct SupabaseLocationRepository {
    url: String,
    key: String,
}

impl SupabaseLocationRepository {
    pub fn new() -> Self {
        Self {
            url: env::var("SUPABASE_URL").expect("SUPABASE_URL env var"),
            key: env::var("SUPABASE_KEY").expect("SUPABASE_KEY env var"),
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let api_key = self.key.parse().unwrap();
        let authorization = format!("Bearer {}", self.key).parse().unwrap();

        headers.insert("apikey", api_key);
        headers.insert("Authorization", authorization);

        headers
    }
}

#[async_trait]
impl LocationRepository for SupabaseLocationRepository {
    async fn fetch_all(&self) -> Result<Vec<Location>, FetchAllError> {
        let client = reqwest::Client::new();
        let res = match client
            .get(format!("{}locations?select=*&order=location", self.url))
            .headers(self.headers())
            .send()
            .await
        {
            Ok(res) => res,
            _ => return Err(FetchAllError::Unknown("could not send request")),
        };

        let supabase_locations = match res.json::<Vec<SupabaseLocation>>().await {
            Ok(locations) => locations,
            _ => return Err(FetchAllError::Unknown("could not parse json")),
        };

        let mut locations = vec![];

        for supabase_location in supabase_locations.into_iter() {
            match supabase_location.try_into() {
                Ok(location) => locations.push(location),
                _ => return Err(FetchAllError::Unknown("could not convert to domain")),
            }
        }

        Ok(locations)
    }
}

#[derive(Deserialize, Serialize)]
pub struct SupabaseLocation {
    location: String,
}

impl TryFrom<SupabaseLocation> for Location {
    type Error = ();

    fn try_from(location: SupabaseLocation) -> Result<Self, Self::Error> {
        Ok(location.location.try_into().or(Err(()))?)
    }
}
