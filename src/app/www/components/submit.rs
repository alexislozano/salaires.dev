use super::palette;
use maud::{html, Markup};

pub fn view(disabled: bool, label: &str) -> Markup {
    html! {
        input
            id="form-submit"
            style=(format!("
                    border: 2px solid {black};
                    border-radius: 4px;
                    padding: 12px;
                    width: 100%;
                    font-weight: bold;
                    font-family: inherit;
                    font-size: inherit;
                    cursor: {cursor};
                    color: {color};
                    border-color: {border_color};
                    background-color: {background_color};",
                background_color=background_color(disabled),
                black=palette::BLACK,
                border_color=border_color(disabled),
                color=color(disabled),
                cursor=cursor(disabled)
            ))
            disabled[disabled]
            type="submit"
            value=(label)
            hx-swap-oob="true";
    }
}

fn background_color(disabled: bool) -> &'static str {
    if disabled {
        palette::WHITE
    } else {
        palette::YELLOW
    }
}

fn border_color(disabled: bool) -> &'static str {
    if disabled {
        palette::GREY
    } else {
        palette::BLACK
    }
}

fn color(disabled: bool) -> &'static str {
    if disabled {
        palette::GREY
    } else {
        palette::BLACK
    }
}

fn cursor(disabled: bool) -> &'static str {
    if disabled {
        "not-allowed"
    } else {
        "pointer"
    }
}
