use crate::infra::{
    CaptchaService, CompanyRepository, LocationRepository, SalaryRepository, TitleRepository,
    TokenRepository, TokenSender,
};
use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Clone)]
pub struct State {
    salary_repo: Arc<dyn SalaryRepository>,
    company_repo: Arc<dyn CompanyRepository>,
    location_repo: Arc<dyn LocationRepository>,
    title_repo: Arc<dyn TitleRepository>,
    captcha_service: Arc<dyn CaptchaService>,
    token_repo: Arc<dyn TokenRepository>,
    token_sender: Arc<dyn TokenSender>,
}

impl State {
    pub fn new(
        salary_repo: Arc<dyn SalaryRepository>,
        company_repo: Arc<dyn CompanyRepository>,
        location_repo: Arc<dyn LocationRepository>,
        title_repo: Arc<dyn TitleRepository>,
        captcha_service: Arc<dyn CaptchaService>,
        token_repo: Arc<dyn TokenRepository>,
        token_sender: Arc<dyn TokenSender>,
    ) -> Self {
        Self {
            salary_repo,
            company_repo,
            location_repo,
            title_repo,
            captcha_service,
            token_repo,
            token_sender,
        }
    }
}

impl FromRef<State> for Arc<dyn SalaryRepository> {
    fn from_ref(state: &State) -> Arc<dyn SalaryRepository> {
        state.salary_repo.clone()
    }
}

impl FromRef<State> for Arc<dyn CompanyRepository> {
    fn from_ref(state: &State) -> Arc<dyn CompanyRepository> {
        state.company_repo.clone()
    }
}

impl FromRef<State> for Arc<dyn LocationRepository> {
    fn from_ref(state: &State) -> Arc<dyn LocationRepository> {
        state.location_repo.clone()
    }
}

impl FromRef<State> for Arc<dyn TitleRepository> {
    fn from_ref(state: &State) -> Arc<dyn TitleRepository> {
        state.title_repo.clone()
    }
}

impl FromRef<State> for Arc<dyn CaptchaService> {
    fn from_ref(state: &State) -> Arc<dyn CaptchaService> {
        state.captcha_service.clone()
    }
}

impl FromRef<State> for Arc<dyn TokenRepository> {
    fn from_ref(state: &State) -> Arc<dyn TokenRepository> {
        state.token_repo.clone()
    }
}

impl FromRef<State> for Arc<dyn TokenSender> {
    fn from_ref(state: &State) -> Arc<dyn TokenSender> {
        state.token_sender.clone()
    }
}
