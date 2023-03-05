use maud::Markup;

use crate::{
    app::www::i18n::I18n,
    domain::models::title::{Error, Title},
};

use super::super::{
    components::select,
    models::form::{Internals, Parsed},
};

pub fn view(internals: Internals<Option<Title>, Error>, titles: Vec<Title>) -> Markup {
    select::view(
        match internals.parsed {
            Parsed::Computed(Err(err)) => match err {
                Error::Empty => Some(I18n::ShouldNotBeEmpty.translate()),
            },
            _ => None,
        },
        "title",
        I18n::Title.translate(),
        titles
            .into_iter()
            .map(|title| String::from(title))
            .collect::<Vec<String>>(),
        I18n::TitlePlaceholder.translate(),
        false,
        "/validate",
        internals.value.as_str(),
    )
}
