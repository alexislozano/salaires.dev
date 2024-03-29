use chrono::Utc;
use serde::Deserialize;

use crate::domain::models::{
    company, compensation, email, level, location, title, xp, Captcha, Company, Compensation,
    Email, Id, Level, Location, Salary, Status, Title, Xp,
};

#[derive(Clone)]
pub enum Parsed<T, E> {
    Init,
    Computed(Result<T, E>),
}

#[derive(Clone)]
pub struct Internals<T, E> {
    pub value: String,
    pub parsed: Parsed<T, E>,
}

impl<T, E> Internals<T, E> {
    fn init() -> Self {
        Self {
            value: String::from(""),
            parsed: Parsed::Init,
        }
    }

    fn computed(value: &str, result: Result<T, E>) -> Self {
        Self {
            value: String::from(value),
            parsed: Parsed::Computed(result),
        }
    }

    fn extract(&self) -> Result<T, ()>
    where
        T: Clone,
    {
        match &self.parsed {
            Parsed::Computed(Ok(t)) => Ok(t.clone()),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UnparsedForm {
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

#[derive(Clone)]
pub struct ParsedForm {
    pub email: Internals<Email, email::Error>,
    pub company: Internals<Company, company::Error>,
    pub title: Internals<Option<Title>, title::Error>,
    pub level: Internals<Option<Level>, level::Error>,
    pub location: Internals<Location, location::Error>,
    pub compensation: Internals<Compensation, compensation::Error>,
    pub company_xp: Internals<Option<Xp>, xp::Error>,
    pub total_xp: Internals<Option<Xp>, xp::Error>,
    pub captcha: Option<Captcha>,
}

impl ParsedForm {
    pub fn init() -> Self {
        Self {
            email: Internals::init(),
            company: Internals::init(),
            title: Internals::init(),
            level: Internals::init(),
            location: Internals::init(),
            compensation: Internals::init(),
            company_xp: Internals::init(),
            total_xp: Internals::init(),
            captcha: None,
        }
    }
}

#[derive(Clone)]
pub struct ValidatedForm {
    email: Email,
    company: Company,
    title: Option<Title>,
    level: Option<Level>,
    location: Location,
    compensation: Compensation,
    company_xp: Option<Xp>,
    total_xp: Option<Xp>,
    captcha: Captcha,
}

impl From<UnparsedForm> for ParsedForm {
    fn from(form: UnparsedForm) -> Self {
        Self {
            email: Internals::computed(form.email.as_str(), Email::try_from(form.email.clone())),
            company: Internals::computed(
                form.company.as_str(),
                Company::try_from(form.company.clone()),
            ),
            title: Internals::computed(
                form.title.as_str(),
                if form.title.is_empty() {
                    Ok(None)
                } else {
                    Title::try_from(form.title.clone()).map(Some)
                },
            ),
            level: Internals::computed(
                form.level.as_str(),
                if form.level.is_empty() {
                    Ok(None)
                } else {
                    Level::try_from(form.level.clone()).map(Some)
                },
            ),
            location: Internals::computed(
                form.location.as_str(),
                Location::try_from(form.location.clone()),
            ),
            compensation: Internals::computed(
                form.compensation.as_str(),
                Compensation::try_from(form.compensation.clone()),
            ),
            company_xp: Internals::computed(
                form.company_xp.as_str(),
                if form.company_xp.is_empty() {
                    Ok(None)
                } else {
                    Xp::try_from(form.company_xp.clone()).map(Some)
                },
            ),
            total_xp: Internals::computed(
                form.total_xp.as_str(),
                if form.total_xp.is_empty() {
                    Ok(None)
                } else {
                    Xp::try_from(form.total_xp.clone()).map(Some)
                },
            ),
            captcha: Captcha::try_from(form.captcha).ok(),
        }
    }
}

impl TryFrom<ParsedForm> for ValidatedForm {
    type Error = ();

    fn try_from(form: ParsedForm) -> Result<Self, Self::Error> {
        Ok(Self {
            email: form.email.extract()?,
            company: form.company.extract()?,
            title: form.title.extract()?,
            level: form.level.extract()?,
            location: form.location.extract()?,
            compensation: form.compensation.extract()?,
            company_xp: form.company_xp.extract()?,
            total_xp: form.total_xp.extract()?,
            captcha: form.captcha.ok_or(())?,
        })
    }
}

impl From<ValidatedForm> for Salary {
    fn from(form: ValidatedForm) -> Self {
        Self::new(
            Id::generate(),
            form.email,
            form.company,
            form.title,
            form.location,
            form.compensation,
            Utc::now().date_naive().into(),
            form.level,
            form.company_xp,
            form.total_xp,
            Status::Waiting,
        )
    }
}

impl From<ValidatedForm> for Captcha {
    fn from(form: ValidatedForm) -> Self {
        form.captcha
    }
}
