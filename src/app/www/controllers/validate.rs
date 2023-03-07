use std::sync::Arc;

use axum::extract::{Form, State};
use futures::future;
use maud::{html, Markup};

use crate::{
    app::www::{
        components::{notification, submit},
        fragments::{
            company_field, company_xp_field, compensation_field, email_field, level_field,
            location_field, title_field, total_xp_field,
        },
        i18n::I18n,
        models::{form::ValidatedForm, ParsedForm, UnparsedForm},
    },
    domain::{
        models::{Company, Location, Title},
        use_cases,
    },
    infra::{CompanyRepository, LocationRepository, TitleRepository},
};

pub async fn post(
    State(company_repo): State<Arc<dyn CompanyRepository>>,
    State(location_repo): State<Arc<dyn LocationRepository>>,
    State(title_repo): State<Arc<dyn TitleRepository>>,
    Form(unparsed_form): Form<UnparsedForm>,
) -> Markup {
    let (companies, locations, titles) = match fetch(company_repo, location_repo, title_repo).await
    {
        Ok((companies, locations, titles)) => (companies, locations, titles),
        _ => return html! { (notification::view(Some(I18n::ValidationError.translate()))) },
    };

    let parsed_form = ParsedForm::from(unparsed_form);
    let disabled = ValidatedForm::try_from(parsed_form.clone()).is_err();

    html! {
        (email_field::view(parsed_form.email))
        (company_field::view(parsed_form.company, companies))
        (title_field::view(parsed_form.title, titles))
        (level_field::view(parsed_form.level))
        (location_field::view(parsed_form.location, locations))
        (compensation_field::view(parsed_form.compensation))
        (company_xp_field::view(parsed_form.company_xp))
        (total_xp_field::view(parsed_form.total_xp))
        (submit::view(disabled, I18n::Send.translate()))
    }
}

async fn fetch(
    company_repo: Arc<dyn CompanyRepository>,
    location_repo: Arc<dyn LocationRepository>,
    title_repo: Arc<dyn TitleRepository>,
) -> Result<(Vec<Company>, Vec<Location>, Vec<Title>), ()> {
    let (companies_result, locations_result, titles_result) = future::join3(
        use_cases::fetch_companies(company_repo),
        use_cases::fetch_locations(location_repo),
        use_cases::fetch_titles(title_repo),
    )
    .await;

    let companies = match companies_result {
        Ok(companies) => companies,
        Err(use_cases::fetch_companies::Error::Unknown(_)) => return Err(()),
    };

    let locations = match locations_result {
        Ok(locations) => locations,
        Err(use_cases::fetch_locations::Error::Unknown(_)) => return Err(()),
    };

    let titles = match titles_result {
        Ok(titles) => titles,
        Err(use_cases::fetch_titles::Error::Unknown(_)) => return Err(()),
    };

    Ok((companies, locations, titles))
}
