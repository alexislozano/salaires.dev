use std::{env, sync::Arc};

use axum::{extract::State, Form};
use futures::future;
use maud::{html, Markup};

use crate::{
    app::www::{
        models::{form::ValidatedForm, ParsedForm, UnparsedForm},
        pages,
    },
    domain::{
        models::{Captcha, Salary},
        use_cases,
    },
    infra::{
        CaptchaService, CompanyRepository, LocationRepository, SalaryRepository, TitleRepository,
        TokenRepository, TokenSender,
    },
};

pub async fn get(
    State(company_repo): State<Arc<dyn CompanyRepository>>,
    State(location_repo): State<Arc<dyn LocationRepository>>,
    State(title_repo): State<Arc<dyn TitleRepository>>,
) -> Markup {
    let hcaptcha_key = env::var("HCAPTCHA_KEY").expect("HCAPTCHA_KEY env var");

    let (companies_result, locations_result, titles_result) = future::join3(
        use_cases::fetch_companies(company_repo),
        use_cases::fetch_locations(location_repo),
        use_cases::fetch_titles(title_repo),
    )
    .await;

    let companies = match companies_result {
        Ok(companies) => companies,
        Err(use_cases::fetch_companies::Error::Unknown(_)) => return html! {},
    };

    let locations = match locations_result {
        Ok(locations) => locations,
        Err(use_cases::fetch_locations::Error::Unknown(_)) => return html! {},
    };

    let titles = match titles_result {
        Ok(titles) => titles,
        Err(use_cases::fetch_titles::Error::Unknown(_)) => return html! {},
    };

    pages::insert::view(hcaptcha_key, companies, titles, locations)
}

pub async fn post(
    State(captcha_service): State<Arc<dyn CaptchaService>>,
    State(salary_repo): State<Arc<dyn SalaryRepository>>,
    State(token_repo): State<Arc<dyn TokenRepository>>,
    State(token_sender): State<Arc<dyn TokenSender>>,
    Form(unparsed_form): Form<UnparsedForm>,
) -> Markup {
    let parsed_form = ParsedForm::from(unparsed_form);
    let validated_form = match ValidatedForm::try_from(parsed_form) {
        Ok(validated_form) => validated_form,
        _ => return html! {},
    };

    let salary = Salary::from(validated_form.clone());
    let captcha = Captcha::from(validated_form);

    match use_cases::insert_salary(
        captcha_service,
        salary_repo,
        token_repo,
        token_sender,
        captcha,
        salary,
    )
    .await
    {
        Ok(()) => html! {},
        _ => html! {},
    }
}
