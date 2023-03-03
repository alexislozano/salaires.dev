use maud::{html, Markup};

use super::super::I18n;
use super::palette;

pub fn view(required: bool, label: &str, sublabel: Option<&str>) -> Markup {
    html!(
        span
            style="
                display: flex;
                justify-content: space-between;
                align-items: center;"
            {
                span
                    style="
                        font-weight: bold;"
                    {
                        ( label )
                        @if !required {
                            span
                                style=(format!("
                                        color: {color};",
                                    color=palette::GREY
                                ))
                                {
                                    (format!(
                                        " - {optional}",
                                        optional=I18n::Optional.translate()
                                    ))
                                }
                        }
                    }
                @if let Some(sub) = sublabel {
                    span
                        style="
                            font-size: 12px;"
                        {
                            (sub)
                        }
                }
            }
    )
}
