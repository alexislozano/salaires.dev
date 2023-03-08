use maud::{html, Markup};

use super::palette;

pub fn view(notification: Option<&str>) -> Markup {
    html! {
        aside
            id="notification"
            style=(format!("
                    display: {display};
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
                display=display(notification),
                background_color=palette::LIGHT_BLUE,
                border_color=palette::DARK_BLUE,
                color=palette::DARK_BLUE,
            ))
            hx-swap-oob="true"
            hx-delete="/notification"
            hx-trigger=(trigger(notification))
            {
                (notification.unwrap_or(""))
            }
    }
}

fn display(notification: Option<&str>) -> &'static str {
    match notification {
        Some(_) => "block",
        None => "none",
    }
}

fn trigger(notification: Option<&str>) -> &'static str {
    match notification {
        Some(_) => "load delay:2s",
        None => "none",
    }
}
