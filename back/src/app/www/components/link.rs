use maud::{html, Markup};

use crate::app::www::components::palette;

pub fn view(label: &str, url: &str) -> Markup {
    html! {
        a
            href=(url)
            style=(format!("
                    border: 2px solid {border_color};
                    border-radius: 4px;
                    font-weight: bold;
                    background-color: {background_color};
                    padding: 12px 16px;
                    text-align: center;
                    color: {color};
                    text-decoration: none;
                    line-height: 1;
                    box-sizing: border-box;
                    white-space: no-wrap;",
                border_color=palette::BLACK,
                background_color=palette::YELLOW,
                color=palette::BLACK
            ))
        {
            (label)
        }
    }
}
