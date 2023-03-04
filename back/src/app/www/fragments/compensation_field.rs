use maud::Markup;

use crate::{
    app::www::i18n::I18n,
    domain::models::compensation::{Compensation, Error},
};

use super::super::components::input;

use super::field::{Internals, Parsed};

pub fn view(internals: Internals<Compensation, Error>) -> Markup {
    input::view(
        match internals.parsed {
            Parsed::Computed(Err(err)) => match err {
                Error::Negative => Some(I18n::ShouldBePositive.translate()),
                Error::NotANumber => Some(I18n::ShouldBeANumber.translate()),
            },
            _ => None,
        },
        "compensation",
        I18n::Compensation.translate(),
        Some(I18n::CompensationHelp.translate()),
        "40000",
        true,
        internals.value.as_str(),
    )
}
