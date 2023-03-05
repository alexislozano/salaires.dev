use maud::{html, Markup};

use super::palette;

pub fn view(notification: Option<&str>) -> Markup {
    html! {
        @match notification {
            Some(message) => {
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
                    hx-delete="/notification"
                    hx-trigger="load delay:2s"
                    hx-swap="outerHTML"
                    {
                        (message)
                    }
            },
            None => {}
        }
    }
}
