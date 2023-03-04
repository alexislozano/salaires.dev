use std::sync::Arc;

use maud::Markup;

use crate::{
    domain::{
        models::{salary::Key, Level, Order, Salary},
        use_cases,
    },
    infra::SalaryRepository,
};

use super::super::{
    components::table::{self, Column, Extract},
    i18n::I18n,
    templates::_500,
};

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

pub async fn view(repo: Arc<dyn SalaryRepository>, order: Order<Key>) -> Markup {
    let salaries = match use_cases::fetch_salaries(repo, order.clone()).await {
        Ok(salaries) => salaries,
        Err(use_cases::fetch_salaries::Error::Unknown(str)) => return _500::view(str),
    };

    let columns = vec![
        build_column(Key::Company, order.clone()),
        build_column(Key::Title, order.clone()),
        build_column(Key::Location, order.clone()),
        build_column(Key::Compensation, order.clone()),
        build_column(Key::CompanyXp, order.clone()),
        build_column(Key::TotalXp, order.clone()),
        build_column(Key::Level, order.clone()),
        build_column(Key::Date, order.clone()),
    ];

    table::view(salaries, columns, order)
}

fn build_column(key: Key, order: Order<Key>) -> Column<Key> {
    let label = match key {
        Key::Company => I18n::Company.translate(),
        Key::Title => I18n::Title.translate(),
        Key::Location => I18n::Location.translate(),
        Key::Compensation => I18n::Compensation.translate(),
        Key::CompanyXp => I18n::CompanyXp.translate(),
        Key::TotalXp => I18n::TotalXp.translate(),
        Key::Level => I18n::Level.translate(),
        Key::Date => I18n::Date.translate(),
    };

    let sublabel = match key {
        Key::Company => "",
        Key::Title => "",
        Key::Location => "",
        Key::Compensation => I18n::CompensationHelp.translate(),
        Key::CompanyXp => I18n::InYears.translate(),
        Key::TotalXp => I18n::InYears.translate(),
        Key::Level => "",
        Key::Date => "",
    };

    let url = if key == order.key {
        format!(
            "?key={key}&direction={direction}",
            key = String::from(key.clone()),
            direction = String::from(order.direction.reverse())
        )
    } else {
        format!("?key={key}", key = String::from(key.clone()))
    };

    let ajax = format!("/sort{url}");

    Column::new(key, label, sublabel, url.as_str(), ajax.as_str())
}
