use serde::Deserialize;

use crate::domain::models::{
    company, compensation, email, level, location, title, xp, Company, Compensation, Email, Level,
    Location, Title, Xp, Captcha,
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

    fn is_valid(&self) -> bool {
        match &self.parsed {
            Parsed::Init => false,
            Parsed::Computed(result) => result.is_ok()
        }
    }
}

pub struct ParsedForm {
    pub email: Internals<Email, email::Error>,
    pub company: Internals<Company, company::Error>,
    pub title: Internals<Option<Title>, title::Error>,
    pub level: Internals<Option<Level>, level::Error>,
    pub location: Internals<Location, location::Error>,
    pub compensation: Internals<Compensation, compensation::Error>,
    pub company_xp: Internals<Option<Xp>, xp::Error>,
    pub total_xp: Internals<Option<Xp>, xp::Error>,
    captcha: Result<Captcha, ()>
}

impl ParsedForm {
    pub fn is_valid(&self) -> bool {
        self.email.is_valid()
            && self.company.is_valid()
            && self.title.is_valid()
            && self.title.is_valid()
            && self.level.is_valid()
            && self.location.is_valid()
            && self.compensation.is_valid()
            && self.company_xp.is_valid()
            && self.total_xp.is_valid()
            && self.captcha.is_ok()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Form {
    email: String,
    company: String,
    title: String,
    location: String,
    compensation: String,
    level: String,
    company_xp: String,
    total_xp: String,
    #[serde(rename = "h-captcha-response")]
    captcha: String,
}

impl From<Form> for ParsedForm {
    fn from(form: Form) -> Self {
        Self {
            email: Internals::new(
                form.email.as_str(),
                Parsed::Computed(Email::try_from(form.email.clone())),
            ),
            company: Internals::new(
                form.company.as_str(),
                Parsed::Computed(Company::try_from(form.company.clone())),
            ),
            title: Internals::new(
                form.title.as_str(),
                Parsed::Computed(if form.title.is_empty() {
                    Ok(None)
                } else {
                    Title::try_from(form.title.clone()).map(Some)
                }),
            ),
            level: Internals::new(
                form.level.as_str(),
                Parsed::Computed(if form.level.is_empty() {
                    Ok(None)
                } else {
                    Level::try_from(form.level.clone()).map(Some)
                }),
            ),
            location: Internals::new(
                form.location.as_str(),
                Parsed::Computed(Location::try_from(form.location.clone())),
            ),
            compensation: Internals::new(
                form.compensation.as_str(),
                Parsed::Computed(Compensation::try_from(form.compensation.clone())),
            ),
            company_xp: Internals::new(
                form.company_xp.as_str(),
                Parsed::Computed(if form.company_xp.is_empty() {
                    Ok(None)
                } else {
                    Xp::try_from(form.company_xp.clone()).map(Some)
                }),
            ),
            total_xp: Internals::new(
                form.total_xp.as_str(),
                Parsed::Computed(if form.total_xp.is_empty() {
                    Ok(None)
                } else {
                    Xp::try_from(form.total_xp.clone()).map(Some)
                }),
            ),
            captcha: Captcha::try_from(form.captcha)
        }
    }
}
