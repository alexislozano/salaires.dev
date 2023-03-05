use maud::Markup;

use crate::{
    app::www::i18n::I18n,
    domain::models::location::{Error, Location},
};

use super::super::components::select;

use super::form::{Internals, Parsed};

pub fn view(internals: Internals<Location, Error>, locations: Vec<Location>) -> Markup {
    select::view(
        match internals.parsed {
            Parsed::Computed(Err(err)) => match err {
                Error::Empty => Some(I18n::ShouldNotBeEmpty.translate()),
            },
            Parsed::Init => Some(""),
            _ => None,
        },
        "location",
        I18n::Location.translate(),
        locations
            .into_iter()
            .map(|location| String::from(location))
            .collect::<Vec<String>>(),
        "Paris",
        true,
        "/validate",
        internals.value.as_str(),
    )
}
