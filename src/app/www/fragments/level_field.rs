use maud::Markup;

use crate::{
    app::www::i18n::I18n,
    domain::models::level::{Error, Level},
};

use super::super::{
    components::dropdown::{self, Choice},
    models::form::{Internals, Parsed},
};

pub fn view(internals: Internals<Option<Level>, Error>) -> Markup {
    let mut choices = vec![Choice::new("", "-")];
    for level in Level::all().iter() {
        let key = String::from(level.clone());
        let label = match level {
            Level::Junior => I18n::Junior.translate(),
            Level::Mid => I18n::Mid.translate(),
            Level::Senior => I18n::Senior.translate(),
        };
        choices.push(Choice::new(key.as_str(), label))
    }

    dropdown::view(
        match internals.parsed {
            Parsed::Computed(Err(err)) => match err {
                Error::NotFound => Some(I18n::LevelIsNotInTheProvidedChoices.translate()),
            },
            _ => None,
        },
        "level",
        I18n::Level.translate(),
        None,
        choices,
        false,
        "/validate",
        internals.value.as_str(),
    )
}
