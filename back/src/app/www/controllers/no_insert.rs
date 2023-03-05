use maud::Markup;

use crate::app::www::{i18n::I18n, pages};

pub async fn get() -> Markup {
    pages::text_only::view(I18n::InsertIsDownForNow.translate())
}
