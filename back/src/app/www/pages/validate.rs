use std::{collections::HashMap, sync::Arc};

use axum::extract::{Query, State};
use maud::Markup;

use crate::{
    app::www::{
        fragments::{
            company_field, company_xp_field, compensation_field, email_field, field::Field,
            level_field, location_field, title_field, total_xp_field,
        },
        templates::_500,
    },
    domain::use_cases,
    infra::{CompanyRepository, LocationRepository, TitleRepository},
};

pub async fn validate(
    State(company_repo): State<Arc<dyn CompanyRepository>>,
    State(location_repo): State<Arc<dyn LocationRepository>>,
    State(title_repo): State<Arc<dyn TitleRepository>>,
    Query(params): Query<HashMap<String, String>>,
) -> Markup {
    match Field::from(params) {
        Field::Email(internals) => email_field::view(internals),
        Field::Company(internals) => {
            let companies = match use_cases::fetch_companies(company_repo).await {
                Ok(companies) => companies,
                Err(use_cases::fetch_companies::Error::Unknown(str)) => return _500::view(str),
            };
            company_field::view(internals, companies)
        }
        Field::Title(internals) => {
            let titles = match use_cases::fetch_titles(title_repo).await {
                Ok(titles) => titles,
                Err(use_cases::fetch_titles::Error::Unknown(str)) => return _500::view(str),
            };
            title_field::view(internals, titles)
        }
        Field::Level(internals) => level_field::view(internals),
        Field::Location(internals) => {
            let locations = match use_cases::fetch_locations(location_repo).await {
                Ok(locations) => locations,
                Err(use_cases::fetch_locations::Error::Unknown(str)) => return _500::view(str),
            };
            location_field::view(internals, locations)
        }
        Field::Compensation(internals) => compensation_field::view(internals),
        Field::CompanyXp(internals) => company_xp_field::view(internals),
        Field::TotalXp(internals) => total_xp_field::view(internals),
        Field::Unknown => _500::view(""),
    }
}
