use maud::{html, Markup};

use super::{label, palette};

pub fn view(
    error: Option<&str>,
    name: &str,
    label: &str,
    sublabel: Option<&str>,
    placeholder: &str,
    required: bool,
    validation_url: &str,
    value: &str,
) -> Markup {
    html! {
        label
            id=(name)
            style="
                display: flex;
                flex-direction: column;
                gap: 4px;
                width: 100%;"
            hx-swap-oob="true"
            {
                (label::view(required, label, sublabel))
                input
                    id=(format!("{name}-input"))
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
                    placeholder=(placeholder)
                    name=(name)
                    hx-post=(validation_url)
                    hx-swap="none";
                span
                    style=(format!("
                            color: {color};",
                        color=palette::RED
                    ))
                    {
                        (error.unwrap_or(" "))
                    }
            }
    }
}

fn border_color(error: Option<&str>) -> &'static str {
    match error {
        Some(_) => palette::RED,
        _ => palette::BLACK,
    }
}
