use maud::Markup;

use crate::domain::models::{salary::Key, Order, Salary};

use super::{super::fragments::salary_table, template};

pub fn view(salaries: Vec<Salary>, order: Order<Key>, notification: Option<&str>) -> Markup {
    template::view(salary_table::view(salaries, order), notification)
}
