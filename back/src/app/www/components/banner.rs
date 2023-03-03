use maud::{html, Markup};

use super::palette;

pub fn view(text: &str) -> Markup {
    html! {
        p
            style=(format!("
                    padding: 12px;
                    margin: 0;
                    border: 1px solid {dark_blue};
                    border-radius: 4px;
                    background-color: {light_blue};
                    color: {dark_blue};",
                dark_blue=palette::DARK_BLUE,
                light_blue=palette::LIGHT_BLUE
            ))
            {
                (text)
            }
    }
}
