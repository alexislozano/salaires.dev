use maud::Markup;

use crate::domain::models::{Order, Salary, salary::Key};

use super::{super::{fragments::salary_table}, template};

pub fn view(salaries: Vec<Salary>, order: Order<Key>) -> Markup {
    template::view(salary_table::view(salaries, order))
}