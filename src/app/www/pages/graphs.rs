use maud::Markup;

use crate::domain::models::{Salary};

use super::{super::fragments::graphs_list, template};

pub fn view(salaries: Vec<Salary>, min_size: usize, notification: Option<&str>) -> Markup {
    template::view(graphs_list::view(salaries, min_size), notification)
}
