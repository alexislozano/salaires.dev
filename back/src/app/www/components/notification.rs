use maud::{html, Markup};

use super::palette;

pub fn view(message: &str) -> Markup {
    html! {
        aside
            style=(format!("
                    position: absolute;
                    z-index: 1;
                    width: 100%;
                    background-color: {background_color};
                    border-bottom: 2px solid {border_color};
                    color: {color};
                    padding: 8px;
                    box-sizing: border-box;
                    font-weight: bold;
                    text-align: center;",
                background_color=palette::LIGHT_BLUE,
                border_color=palette::DARK_BLUE,
                color=palette::DARK_BLUE,
            ))
            {
                (message)
            }
    }
}
