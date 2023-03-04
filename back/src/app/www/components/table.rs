use maud::{html, Markup};

use crate::domain::models::{Direction, Order};

use super::super::components::palette;

pub struct Column<K> {
    key: K,
    label: String,
    sublabel: String,
    link: String,
}

impl<K> Column<K> {
    pub fn new(key: K, label: &str, sublabel: &str, link: &str) -> Self {
        Self {
            key,
            label: String::from(label),
            sublabel: String::from(sublabel),
            link: String::from(link),
        }
    }
}

pub trait Extract<K> {
    fn extract(&self, key: K) -> String;
}

pub fn view<T, K>(items: Vec<T>, columns: Vec<Column<K>>, order: Order<K>) -> Markup
where
    K: Clone + PartialEq,
    T: Extract<K>,
{
    html! {
        table
            style="
                border-spacing: 0;
                width: 100%;"
            {
                (head(&columns, &order))
                (body(&items, &columns))
            }
    }
}

fn head<K>(columns: &Vec<Column<K>>, order: &Order<K>) -> Markup
where
    K: PartialEq,
{
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
                                a
                                    style=(format!("
                                            height: 48px;
                                            width: 100%;
                                            border: 0;
                                            background-color: transparent;
                                            padding: 8px 16px;
                                            text-align: start;
                                            cursor: pointer;
                                            font-size: inherit;
                                            font-family: inherit;
                                            white-space: nowrap;
                                            display: flex;
                                            flex-direction: column;
                                            box-sizing: border-box;
                                            color: {color};
                                            text-decoration: none;",
                                        color=palette::BLACK
                                    ))
                                    href=(column.link)
                                    {
                                        span
                                            style="
                                                font-weight: bold;"
                                            {
                                                @if column.key == order.key {
                                                    (format!(
                                                        "{label} {arrow}",
                                                        label=column.label,
                                                        arrow=match order.direction {
                                                            Direction::Asc => "↑",
                                                            Direction::Desc => "↓"
                                                        }
                                                    ))
                                                } @else {
                                                    (column.label)
                                                }
                                            }
                                        span
                                            style="
                                                font-size: 12px;
                                                font-weight: normal;"
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
                                    white-space: nowrap;"
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
