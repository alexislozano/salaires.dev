use std::{env, sync::Arc};

use futures::future;
use axum::extract::State;
use maud::Markup;

use crate::{
    app::www::{
        components::{banner, button, hcaptcha},
        fragments::{
            company_field, company_xp_field, compensation_field, email_field, field, level_field,
            location_field, title_field, total_xp_field,
        },
        i18n::I18n,
    },
    domain::use_cases,
    infra::{CompanyRepository, LocationRepository, TitleRepository},
};

use super::super::components::form;
use super::super::templates::{page, _500};

pub async fn insert(
    State(company_repo): State<Arc<dyn CompanyRepository>>,
    State(location_repo): State<Arc<dyn LocationRepository>>,
    State(title_repo): State<Arc<dyn TitleRepository>>,
) -> Markup {
    let hcaptcha_key = env::var("HCAPTCHA_KEY").expect("HCAPTCHA_KEY env var");

    let (companies_result, locations_result, titles_result) = future::join3(
        use_cases::fetch_companies(company_repo),
        use_cases::fetch_locations(location_repo),
        use_cases::fetch_titles(title_repo),
    ).await;

    let companies = match companies_result {
        Ok(companies) => companies,
        Err(use_cases::fetch_companies::Error::Unknown(str)) => return _500::view(str),
    };

    let locations = match locations_result {
        Ok(locations) => locations,
        Err(use_cases::fetch_locations::Error::Unknown(str)) => return _500::view(str),
    };

    let titles = match titles_result {
        Ok(titles) => titles,
        Err(use_cases::fetch_titles::Error::Unknown(str)) => return _500::view(str),
    };

    let elements = vec![
        banner::view(I18n::EmailExplanation.translate()),
        email_field::view(field::Internals::new("", field::Parsed::Init)),
        company_field::view(field::Internals::new("", field::Parsed::Init), companies),
        title_field::view(field::Internals::new("", field::Parsed::Init), titles),
        level_field::view(field::Internals::new("", field::Parsed::Init)),
        location_field::view(field::Internals::new("", field::Parsed::Init), locations),
        compensation_field::view(field::Internals::new("", field::Parsed::Init)),
        company_xp_field::view(field::Internals::new("", field::Parsed::Init)),
        total_xp_field::view(field::Internals::new("", field::Parsed::Init)),
        hcaptcha::view(hcaptcha_key.as_str()),
    ];

    page::view(form::view(I18n::IAddMySalary.translate(), elements))
}
