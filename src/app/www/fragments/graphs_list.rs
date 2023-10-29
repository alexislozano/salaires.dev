use std::collections::HashMap;

use maud::Markup;

use crate::{
    app::www::components::graph::{self, Dataset, Graph},
    domain::models::Salary,
};

use super::super::i18n::I18n;

pub fn view(salaries: Vec<Salary>, min_size: usize) -> Markup {
    let mut by_city: HashMap<String, Vec<(i32, i32, String)>> = HashMap::new();
    let mut by_company: HashMap<String, Vec<(i32, i32, String)>> = HashMap::new();
    for s in salaries.clone() {
        if let Ok(loc) = s.location.try_into() {
            if let Ok(company_name) = s.company.try_into() {
                if let Some(xp) = s.total_xp {
                    let loc: String = loc;
                    let company_name: String = company_name;
                    let xp: i32 = xp.into();
                    let comp: i32 = s.compensation.into();
                    by_city.entry(loc.clone()).or_insert(vec![]).push((
                        xp,
                        comp,
                        company_name.clone(),
                    ));

                    by_company
                        .entry(company_name)
                        .or_insert(vec![])
                        .push((xp, comp, loc));
                }
            }
        }
    }

    let scatter_exp_salary_city = Graph::new(
        "exp_vs_salary_by_city",
        "scatter",
        by_city
            .into_iter()
            .filter(|(_, v)| v.len() >= min_size)
            .map(|(city_name, values)| Dataset::new(&city_name, values))
            .collect(),
        &format!(
            "{} ({})",
            I18n::TotalXp.translate(),
            I18n::InYears.translate(),
        ),
        I18n::Compensation.translate(),
        I18n::CompensationVsTotalXpByCityTitle.translate(),
        Some(I18n::CompensationVsTotalXpByCitySubTitle.translate()),
    );

    let scatter_exp_salary_company = Graph::new(
        "exp_vs_salary_by_company",
        "scatter",
        by_company
            .into_iter()
            .filter(|(_, v)| v.len() >= min_size)
            .map(|(cname, values)| Dataset::new(&cname, values))
            .collect(),
        &format!(
            "{} ({})",
            I18n::TotalXp.translate(),
            I18n::InYears.translate(),
        ),
        I18n::Compensation.translate(),
        I18n::CompensationVsTotalXpByCompanyTitle.translate(),
        Some(I18n::CompensationVsTotalXpByCompanySubTitle.translate()),
    );

    graph::view(
        min_size,
        vec![scatter_exp_salary_city, scatter_exp_salary_company],
    )
}
