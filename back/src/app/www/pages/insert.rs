use std::sync::Arc;

use axum::extract::State;
use maud::Markup;

use crate::{
    app::www::{
        components::{
            banner,
            dropdown::{self, Choice},
            select,
        },
        i18n::I18n,
    },
    domain::{models::Level, use_cases},
    infra::{CompanyRepository, LocationRepository, TitleRepository},
};

use super::super::components::{form, input};
use super::super::templates::{page, _500};

pub async fn insert(
    State(company_repo): State<Arc<dyn CompanyRepository>>,
    State(location_repo): State<Arc<dyn LocationRepository>>,
    State(title_repo): State<Arc<dyn TitleRepository>>,
) -> Markup {
    let companies = match use_cases::fetch_companies(company_repo).await {
        Ok(companies) => companies,
        Err(use_cases::fetch_companies::Error::Unknown(str)) => return _500::view(str),
    };

    let locations = match use_cases::fetch_locations(location_repo).await {
        Ok(locations) => locations,
        Err(use_cases::fetch_locations::Error::Unknown(str)) => return _500::view(str),
    };

    let titles = match use_cases::fetch_titles(title_repo).await {
        Ok(titles) => titles,
        Err(use_cases::fetch_titles::Error::Unknown(str)) => return _500::view(str),
    };

    let choices = Level::all()
        .iter()
        .map(|level| {
            let key = String::from(level.clone());
            let label = match level {
                Level::Junior => I18n::Junior.translate(),
                Level::Mid => I18n::Mid.translate(),
                Level::Senior => I18n::Senior.translate(),
            };
            Choice::new(key.as_str(), label)
        })
        .collect::<Vec<Choice>>();

    let elements = vec![
        banner::view(I18n::EmailExplanation.translate()),
        input::view(
            None,
            I18n::Email.translate(),
            None,
            "moi@exemple.fr",
            true,
            "email",
        ),
        select::view(
            None,
            "companies",
            I18n::Company.translate(),
            companies
                .into_iter()
                .map(|company| String::from(company))
                .collect::<Vec<String>>(),
            "Google",
            true,
            "",
        ),
        select::view(
            None,
            "titles",
            I18n::Title.translate(),
            titles
                .into_iter()
                .map(|title| String::from(title))
                .collect::<Vec<String>>(),
            I18n::TitlePlaceholder.translate(),
            false,
            "",
        ),
        dropdown::view(
            None,
            I18n::Level.translate(),
            None,
            choices,
            false,
            String::from(Level::Junior).as_str(),
        ),
        select::view(
            None,
            "locations",
            I18n::Location.translate(),
            locations
                .into_iter()
                .map(|location| String::from(location))
                .collect::<Vec<String>>(),
            "Paris",
            true,
            "",
        ),
        input::view(
            None,
            I18n::Compensation.translate(),
            Some(I18n::CompensationHelp.translate()),
            "40000",
            true,
            "40000",
        ),
        input::view(
            None,
            I18n::CompanyXp.translate(),
            Some(I18n::InYears.translate()),
            "2",
            false,
            "2",
        ),
        input::view(
            None,
            I18n::TotalXp.translate(),
            Some(I18n::InYears.translate()),
            "10",
            true,
            "10",
        ),
    ];

    page::view(form::view(I18n::IAddMySalary.translate(), elements))
}
