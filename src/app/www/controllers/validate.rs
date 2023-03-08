use std::sync::Arc;

use axum::{
    extract::{Form, State},
    http::HeaderMap,
};
use maud::{html, Markup};

use crate::{
    app::www::{
        components::{notification, submit},
        fragments::{
            company_field, company_xp_field, compensation_field, email_field, level_field,
            location_field, title_field, total_xp_field,
        },
        i18n::I18n,
        models::{HXTriggerName, ParsedForm, UnparsedForm, ValidatedForm},
    },
    domain::use_cases,
    infra::{CompanyRepository, LocationRepository, TitleRepository},
};

pub async fn post(
    headers: HeaderMap,
    State(company_repo): State<Arc<dyn CompanyRepository>>,
    State(location_repo): State<Arc<dyn LocationRepository>>,
    State(title_repo): State<Arc<dyn TitleRepository>>,
    Form(unparsed_form): Form<UnparsedForm>,
) -> Markup {
    let hx_trigger_name = match HXTriggerName::try_from(headers) {
        Ok(hx_trigger_name) => hx_trigger_name,
        _ => return error(),
    };

    let parsed_form = ParsedForm::from(unparsed_form);
    let disabled = ValidatedForm::try_from(parsed_form.clone()).is_err();

    let field = match (&hx_trigger_name).into() {
        "email" => email_field::view(parsed_form.email),
        "company" => match use_cases::fetch_companies(company_repo).await {
            Ok(companies) => company_field::view(parsed_form.company, companies),
            Err(use_cases::fetch_companies::Error::Unknown(_)) => return error(),
        },
        "title" => match use_cases::fetch_titles(title_repo).await {
            Ok(titles) => title_field::view(parsed_form.title, titles),
            Err(use_cases::fetch_titles::Error::Unknown(_)) => return error(),
        },
        "level" => level_field::view(parsed_form.level),
        "location" => match use_cases::fetch_locations(location_repo).await {
            Ok(locations) => location_field::view(parsed_form.location, locations),
            Err(use_cases::fetch_locations::Error::Unknown(_)) => return error(),
        },
        "compensation" => compensation_field::view(parsed_form.compensation),
        "company_xp" => company_xp_field::view(parsed_form.company_xp),
        "total_xp" => total_xp_field::view(parsed_form.total_xp),
        _ => html! {},
    };

    html! {
        (field)
        (submit::view(disabled, I18n::Send.translate()))
    }
}

fn error() -> Markup {
    html! { (notification::view(Some(I18n::ValidationError.translate()))) }
}
