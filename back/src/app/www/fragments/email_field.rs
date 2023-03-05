use maud::Markup;

use crate::{
    app::www::i18n::I18n,
    domain::models::email::{Email, Error},
};

use super::super::components::input;

use super::form::{Internals, Parsed};

pub fn view(internals: Internals<Email, Error>) -> Markup {
    input::view(
        match internals.parsed {
            Parsed::Computed(Err(err)) => match err {
                Error::NotPro => Some(I18n::EmailShouldBePro.translate()),
                Error::NotContainingAnAt => Some(I18n::EmailShouldContainAnAt.translate()),
            },
            Parsed::Init => Some(""),
            _ => None,
        },
        "email",
        I18n::Email.translate(),
        None,
        "moi@exemple.fr",
        true,
        "/validate",
        internals.value.as_str(),
    )
}
