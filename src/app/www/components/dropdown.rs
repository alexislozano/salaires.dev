use maud::{html, Markup};

use super::{label, palette};

pub struct Choice {
    key: String,
    label: String,
}

impl Choice {
    pub fn new(key: &str, label: &str) -> Self {
        Self {
            key: String::from(key),
            label: String::from(label),
        }
    }
}

pub fn view(
    error: Option<&str>,
    name: &str,
    label: &str,
    sublabel: Option<&str>,
    choices: Vec<Choice>,
    required: bool,
    validation_url: &str,
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
            hx-swap-oob="true"
            {
                (label::view(required, label, sublabel))
                select
                    id=(format!("{name}-select"))
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
                    name=(name)
                    hx-post=(validation_url)
                    hx-swap="none"
                    {
                        @for choice in choices {
                            option
                                value=(choice.key)
                                selected[choice.key == value]
                                {
                                    (choice.label)
                                }
                        }
                    }
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
        None => palette::BLACK,
    }
}
