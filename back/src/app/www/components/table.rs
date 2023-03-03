use maud::{html, Markup};

use super::super::components::palette;

pub struct Column<K> {
    key: K,
    label: String,
    sublabel: String,
}

impl<K> Column<K> {
    pub fn new(key: K, label: &str, sublabel: &str) -> Self {
        Self {
            key,
            label: String::from(label),
            sublabel: String::from(sublabel),
        }
    }
}

pub trait Extract<K> {
    fn extract(&self, key: K) -> String;
}

pub fn view<T, K>(items: Vec<T>, columns: Vec<Column<K>>) -> Markup
where
    K: Clone,
    T: Extract<K>,
{
    html! {
        table
            style="
                border-spacing: 0;
                width: 100%;"
            {
                (head(&columns))
                (body(&items, &columns))
            }
    }
}

fn head<K>(columns: &Vec<Column<K>>) -> Markup {
    html! {
        thead
            style="
                position: sticky;
                top: 0;"
            {
                tr {
                    @for column in columns {
                        th
                            style=(format!("
                                    padding: 0;
                                    background-color: {background_color};
                                    border-bottom: 2px solid;",
                                background_color=palette::SAND
                            ))
                            {
                                button
                                    style="
                                        height: 48px;
                                        width: 100%;
                                        border: 0;
                                        background-color: transparent;
                                        padding: 8px 16px;
                                        text-align: start;
                                        cursor: pointer;
                                        font-size: inherit;
                                        font-family: inherit;
                                        white-space: no-wrap;
                                        display: flex;
                                        flex-direction: column;"
                                    {
                                        span
                                            style="
                                                font-weight: bold;"
                                            {
                                                (column.label)
                                            }
                                        span
                                            style="
                                                font-size: 12px;"
                                            {
                                                (column.sublabel)
                                            }
                                    }
                            }
                        }
                }
            }
    }
}

fn body<T: Extract<K>, K>(items: &Vec<T>, columns: &Vec<Column<K>>) -> Markup
where
    K: Clone,
    T: Extract<K>,
{
    html! {
        tbody {
            @for (index, item) in items.iter().enumerate() {
                tr
                    style=(format!("
                            background-color: {background_color};",
                        background_color=background_color(index)
                    ))
                    {
                        @for column in columns {
                            td
                                style="
                                    height: 48px;
                                    padding: 0 16px;
                                    white-space: no-wrap;"
                                {
                                    (item.extract(column.key.clone()))
                                }
                        }
                    }
            }
        }
    }
}

fn background_color(index: usize) -> &'static str {
    if index % 2 == 0 {
        palette::LIGHT_SAND
    } else {
        palette::SAND
    }
}
