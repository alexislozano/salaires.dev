use std::sync::Arc;

use axum::extract::{Form, State};
use maud::{html, Markup};

use crate::{
    app::www::models::{form::ValidatedForm, ParsedForm, UnparsedForm},
    domain::{
        models::{Captcha, Salary},
        use_cases,
    },
    infra::{CaptchaService, SalaryRepository, TokenRepository, TokenSender},
};

pub async fn salaries(
    State(captcha_service): State<Arc<dyn CaptchaService>>,
    State(salary_repo): State<Arc<dyn SalaryRepository>>,
    State(token_repo): State<Arc<dyn TokenRepository>>,
    State(token_sender): State<Arc<dyn TokenSender>>,
    Form(unparsed_form): Form<UnparsedForm>,
) -> Markup {
    let parsed_form = ParsedForm::from(unparsed_form);
    let validated_form = match ValidatedForm::try_from(parsed_form) {
        Ok(validated_form) => validated_form,
        _ => return html! {},
    };

    let salary = Salary::from(validated_form.clone());
    let captcha = Captcha::from(validated_form);

    match use_cases::insert_salary(
        captcha_service,
        salary_repo,
        token_repo,
        token_sender,
        captcha,
        salary,
    )
    .await
    {
        Ok(()) => html! {},
        _ => html! {},
    }
}
