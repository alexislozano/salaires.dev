use maud::Markup;

use crate::{
    app::www::{
        components::{banner, form, hcaptcha, submit},
        fragments::{
            company_field, company_xp_field, compensation_field, email_field, level_field,
            location_field, title_field, total_xp_field,
        },
        i18n::I18n,
        models,
    },
    domain::models::{Company, Location, Title},
};

use super::template;

pub fn view(
    hcaptcha_key: String,
    companies: Vec<Company>,
    locations: Vec<Location>,
    titles: Vec<Title>,
) -> Markup {
    let elements = vec![
        banner::view(I18n::EmailExplanation.translate()),
        email_field::view(models::form::Internals::new("", models::form::Parsed::Init)),
        company_field::view(
            models::form::Internals::new("", models::form::Parsed::Init),
            companies,
        ),
        title_field::view(
            models::form::Internals::new("", models::form::Parsed::Init),
            titles,
        ),
        level_field::view(models::form::Internals::new("", models::form::Parsed::Init)),
        location_field::view(
            models::form::Internals::new("", models::form::Parsed::Init),
            locations,
        ),
        compensation_field::view(models::form::Internals::new("", models::form::Parsed::Init)),
        company_xp_field::view(models::form::Internals::new("", models::form::Parsed::Init)),
        total_xp_field::view(models::form::Internals::new("", models::form::Parsed::Init)),
        hcaptcha::view(hcaptcha_key.as_str()),
        submit::view(true, I18n::Send.translate()),
    ];

    template::view(form::view(
        I18n::IAddMySalary.translate(),
        "/insert",
        elements,
    ))
}
