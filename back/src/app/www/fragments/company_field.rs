use maud::Markup;

use crate::{
    app::www::i18n::I18n,
    domain::models::company::{Company, Error},
};

use super::super::components::select;

use super::form::{Internals, Parsed};

pub fn view(internals: Internals<Company, Error>, companies: Vec<Company>) -> Markup {
    select::view(
        match internals.parsed {
            Parsed::Computed(Err(err)) => match err {
                Error::Empty => Some(I18n::ShouldNotBeEmpty.translate()),
            },
            Parsed::Init => Some(""),
            _ => None,
        },
        "company",
        I18n::Company.translate(),
        companies
            .into_iter()
            .map(|company| String::from(company))
            .collect::<Vec<String>>(),
        "Google",
        true,
        "/validate",
        internals.value.as_str(),
    )
}
