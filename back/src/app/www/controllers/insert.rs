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
        models::{Captcha, Company, Location, Salary, Title},
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
    let (hcaptcha_key, companies, locations, titles) =
        match fetch(company_repo, location_repo, title_repo).await {
            Ok((hcaptcha_key, companies, locations, titles)) => {
                (hcaptcha_key, companies, locations, titles)
            }
            _ => return html! {},
        };

    pages::insert::view(
        ParsedForm::init(),
        hcaptcha_key,
        companies,
        locations,
        titles,
    )
}

pub async fn post(
    State(company_repo): State<Arc<dyn CompanyRepository>>,
    State(location_repo): State<Arc<dyn LocationRepository>>,
    State(title_repo): State<Arc<dyn TitleRepository>>,
    State(captcha_service): State<Arc<dyn CaptchaService>>,
    State(salary_repo): State<Arc<dyn SalaryRepository>>,
    State(token_repo): State<Arc<dyn TokenRepository>>,
    State(token_sender): State<Arc<dyn TokenSender>>,
    Form(unparsed_form): Form<UnparsedForm>,
) -> Markup {
    let (hcaptcha_key, companies, locations, titles) =
        match fetch(company_repo, location_repo, title_repo).await {
            Ok((hcaptcha_key, companies, locations, titles)) => {
                (hcaptcha_key, companies, locations, titles)
            }
            _ => return html! {},
        };

    let parsed_form = ParsedForm::from(unparsed_form);
    let validated_form = match ValidatedForm::try_from(parsed_form.clone()) {
        Ok(validated_form) => validated_form,
        _ => return pages::insert::view(parsed_form, hcaptcha_key, companies, locations, titles),
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
        Ok(()) => pages::insert::view(
            ParsedForm::init(),
            hcaptcha_key,
            companies,
            locations,
            titles,
        ),
        _ => pages::insert::view(parsed_form, hcaptcha_key, companies, locations, titles),
    }
}

async fn fetch(
    company_repo: Arc<dyn CompanyRepository>,
    location_repo: Arc<dyn LocationRepository>,
    title_repo: Arc<dyn TitleRepository>,
) -> Result<(String, Vec<Company>, Vec<Location>, Vec<Title>), ()> {
    let hcaptcha_key = env::var("HCAPTCHA_KEY").expect("HCAPTCHA_KEY env var");

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

    Ok((hcaptcha_key, companies, locations, titles))
}
