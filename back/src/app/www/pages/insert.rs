use maud::Markup;

use super::template;

pub fn view(main: Markup) -> Markup {
    template::view(main)
}