use maud::Markup;

use crate::{
    app::www::{
        components::{banner, form, hcaptcha, submit},
        fragments::{
            company_field, company_xp_field, compensation_field, email_field, level_field,
            location_field, title_field, total_xp_field,
        },
        i18n::I18n,
        models::ParsedForm,
    },
    domain::models::{Company, Location, Title},
};

use super::template;

pub fn view(
    form: ParsedForm,
    hcaptcha_key: String,
    companies: Vec<Company>,
    locations: Vec<Location>,
    titles: Vec<Title>,
    notification: Option<&str>,
) -> Markup {
    let elements = vec![
        banner::view(I18n::EmailExplanation.translate()),
        email_field::view(form.email),
        company_field::view(form.company, companies),
        title_field::view(form.title, titles),
        level_field::view(form.level),
        location_field::view(form.location, locations),
        compensation_field::view(form.compensation),
        company_xp_field::view(form.company_xp),
        total_xp_field::view(form.total_xp),
        hcaptcha::view(hcaptcha_key.as_str()),
        submit::view(true, I18n::Send.translate()),
    ];

    template::view(
        form::view(I18n::IAddMySalary.translate(), "/insert", elements),
        notification,
    )
}
