use maud::Markup;

use crate::{
    app::www::i18n::I18n,
    domain::models::company_type::{Error, CompanyType},
};

use super::super::{
    components::dropdown::{self, Choice},
    models::form::{Internals, Parsed},
};

pub fn view(internals: Internals<Option<CompanyType>, Error>) -> Markup {
    let mut choices = vec![Choice::new("", "-")];
    for company_type in CompanyType::all().iter() {
        let key = String::from(company_type.clone());
        let label = key.as_str();
        choices.push(Choice::new(key.as_str(), label))
    }

    dropdown::view(
        match internals.parsed {
            Parsed::Computed(Err(err)) => match err {
                Error::NotFound => Some(I18n::CompanyTypeIsNotInTheProvidedChoices.translate()),
            },
            _ => None,
        },
        "company_type",
        I18n::CompanyType.translate(),
        None,
        choices,
        false,
        "/validate",
        internals.value.as_str(),
    )
}
