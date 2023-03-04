use std::collections::HashMap;

use crate::domain::models::{
    company, compensation, email, level, location, title, xp, Company, Compensation, Email, Level,
    Location, Title, Xp,
};

pub enum Parsed<T, E> {
    Init,
    Computed(Result<T, E>),
}

pub struct Internals<T, E> {
    pub value: String,
    pub parsed: Parsed<T, E>,
}

impl<T, E> Internals<T, E> {
    pub fn new(value: &str, parsed: Parsed<T, E>) -> Self {
        Self {
            value: String::from(value),
            parsed,
        }
    }
}

pub enum Field {
    Email(Internals<Email, email::Error>),
    Company(Internals<Company, company::Error>),
    Title(Internals<Title, title::Error>),
    Level(Internals<Level, level::Error>),
    Location(Internals<Location, location::Error>),
    Compensation(Internals<Compensation, compensation::Error>),
    CompanyXp(Internals<Xp, xp::Error>),
    TotalXp(Internals<Xp, xp::Error>),
    Unknown,
}

impl From<HashMap<String, String>> for Field {
    fn from(params: HashMap<String, String>) -> Self {
        match params.get("email") {
            Some(email) => {
                return Self::Email(Internals::new(
                    email,
                    Parsed::Computed(Email::try_from(String::from(email))),
                ))
            }
            None => {}
        }

        match params.get("company") {
            Some(company) => {
                return Self::Company(Internals::new(
                    company,
                    Parsed::Computed(Company::try_from(String::from(company))),
                ))
            }
            None => {}
        }

        match params.get("title") {
            Some(title) => {
                return Self::Title(Internals::new(
                    title,
                    Parsed::Computed(Title::try_from(String::from(title))),
                ))
            }
            None => {}
        }

        match params.get("level") {
            Some(level) => {
                return Self::Level(Internals::new(
                    level,
                    Parsed::Computed(Level::try_from(String::from(level))),
                ))
            }
            None => {}
        }

        match params.get("location") {
            Some(location) => {
                return Self::Location(Internals::new(
                    location,
                    Parsed::Computed(Location::try_from(String::from(location))),
                ))
            }
            None => {}
        }

        match params.get("compensation") {
            Some(compensation) => {
                return Self::Compensation(Internals::new(
                    compensation,
                    Parsed::Computed(Compensation::try_from(String::from(compensation))),
                ))
            }
            None => {}
        }

        match params.get("company_xp") {
            Some(xp) => {
                return Self::CompanyXp(Internals::new(
                    xp,
                    Parsed::Computed(Xp::try_from(String::from(xp))),
                ))
            }
            None => {}
        }

        match params.get("total_xp") {
            Some(xp) => {
                return Self::TotalXp(Internals::new(
                    xp,
                    Parsed::Computed(Xp::try_from(String::from(xp))),
                ))
            }
            None => {}
        }

        Self::Unknown
    }
}
