use maud::{html, Markup};

use super::{label, palette};

pub fn view(
    error: Option<&str>,
    label: &str,
    sublabel: Option<&str>,
    placeholder: &str,
    required: bool,
    value: &str,
) -> Markup {
    html! {
        label
            style="
                display: flex;
                flex-direction: column;
                gap: 4px;
                width: 100%;"
            {
                (label::view(required, label, sublabel))
                input
                    style=(format!("
                            border: 2px solid {border_color};
                            border-radius: 4px;
                            font-family: inherit;
                            font-size: inherit;
                            padding: 12px;
                            font-weight: bold;",
                        border_color=border_color(error)
                    ))
                    type="text"
                    value=(value)
                    placeholder=(placeholder);
            }
    }
}

fn border_color(error: Option<&str>) -> &'static str {
    match error {
        Some(_) => palette::RED,
        _ => palette::BLACK,
    }
}
