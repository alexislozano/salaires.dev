use std::sync::Arc;

use axum::extract::State;
use maud::Markup;

use crate::{
    domain::{
        models::{Level, Salary},
        use_cases,
    },
    infra::SalaryRepository,
};

use super::super::{
    components::table::{self, Column, Extract},
    i18n::I18n,
    templates::{page, _500},
};

#[derive(Clone)]
enum Key {
    Company,
    Title,
    Location,
    Compensation,
    CompanyXp,
    TotalXp,
    Level,
    Date,
}

impl Extract<Key> for Salary {
    fn extract(&self, key: Key) -> String {
        match key {
            Key::Company => self.company.clone().into(),
            Key::Title => self
                .title
                .clone()
                .map(|x| String::from(x))
                .unwrap_or(String::from("")),
            Key::Location => self.location.clone().into(),
            Key::Compensation => self.compensation.clone().into(),
            Key::CompanyXp => self
                .company_xp
                .clone()
                .map(|x| String::from(x))
                .unwrap_or(String::from("")),
            Key::TotalXp => self
                .total_xp
                .clone()
                .map(|x| String::from(x))
                .unwrap_or(String::from("")),
            Key::Level => self
                .level
                .clone()
                .map(|x| {
                    String::from(match x {
                        Level::Junior => I18n::Junior.translate(),
                        Level::Mid => I18n::Mid.translate(),
                        Level::Senior => I18n::Senior.translate(),
                    })
                })
                .unwrap_or(String::from("")),
            Key::Date => self.date.clone().into(),
        }
    }
}

pub async fn index(State(repo): State<Arc<dyn SalaryRepository>>) -> Markup {
    let salaries = match use_cases::fetch_salaries(repo).await {
        Ok(salaries) => salaries,
        Err(use_cases::fetch_salaries::Error::Unknown(str)) => return _500::view(str),
    };

    let columns = vec![
        Column::new(Key::Company, I18n::Company.translate(), ""),
        Column::new(Key::Title, I18n::Title.translate(), ""),
        Column::new(Key::Location, I18n::Location.translate(), ""),
        Column::new(
            Key::Compensation,
            I18n::Compensation.translate(),
            I18n::CompensationHelp.translate(),
        ),
        Column::new(
            Key::CompanyXp,
            I18n::CompanyXp.translate(),
            I18n::InYears.translate(),
        ),
        Column::new(
            Key::TotalXp,
            I18n::TotalXp.translate(),
            I18n::InYears.translate(),
        ),
        Column::new(Key::Level, I18n::Level.translate(), ""),
        Column::new(Key::Date, I18n::Date.translate(), ""),
    ];

    page::view(table::view(salaries, columns))
}
