use maud::Markup;

use crate::{
    app::www::i18n::I18n,
    domain::models::xp::{Error, Xp},
};

use super::super::components::input;

use super::field::{Internals, Parsed};

pub fn view(internals: Internals<Option<Xp>, Error>) -> Markup {
    input::view(
        match internals.parsed {
            Parsed::Computed(Err(err)) => match err {
                Error::Negative => Some(I18n::ShouldBePositive.translate()),
                Error::NotANumber => Some(I18n::ShouldBeANumber.translate()),
            },
            _ => None,
        },
        "company_xp",
        I18n::CompanyXp.translate(),
        Some(I18n::InYears.translate()),
        "2",
        false,
        internals.value.as_str(),
    )
}
