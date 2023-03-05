use maud::{html, Markup};

use crate::app::www::components::{label, palette};

pub fn view(
    error: Option<&str>,
    name: &str,
    label: &str,
    options: Vec<String>,
    placeholder: &str,
    required: bool,
    value: &str,
) -> Markup {
    html! {
        label
            id=(name)
            style=(format!("
                    display: flex;
                    flex-direction: column;
                    gap: 4px;
                    width: 100%;"
            ))
            {
                (label::view(required, label, None))
                input
                    id=(format!("{name}-input"))
                    style=(format!("
                            border: 2px solid {border_color};
                            border-radius: 4px;
                            font-family: inherit;
                            font-size: inherit;
                            padding: 12px;
                            font-weight: bold;
                            background-color: {background_color};",
                        border_color=border_color(error),
                        background_color=palette::WHITE
                    ))
                    list=(format!("{name}-list"))
                    placeholder=(placeholder)
                    value=(value)
                    name=(name)
                    hx-get="/validate"
                    hx-target=(format!("#{name}"));
                span
                    style=(format!("
                            color: {color};",
                        color=palette::RED
                    ))
                    {
                        (error.unwrap_or(" "))
                    }
                datalist
                    id=(format!("{name}-list"))
                    {
                        @for option in options {
                            option
                                value=(option);
                        }
                    }
            }
    }
}

fn border_color(error: Option<&str>) -> &'static str {
    match error {
        Some(_) => palette::RED,
        None => palette::BLACK,
    }
}
