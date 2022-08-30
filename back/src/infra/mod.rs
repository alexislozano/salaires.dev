pub mod captcha_service;
pub mod company_repository;
pub mod location_repository;
pub mod salary_repository;
pub mod title_repository;

pub use captcha_service::CaptchaService;
pub use captcha_service::{hcaptcha::HCaptchaService, stub::StubCaptchaService};

pub use company_repository::CompanyRepository;
pub use company_repository::{
    in_memory::InMemoryCompanyRepository, supabase::SupabaseCompanyRepository,
};

pub use location_repository::LocationRepository;
pub use location_repository::{
    in_memory::InMemoryLocationRepository, supabase::SupabaseLocationRepository,
};

pub use title_repository::TitleRepository;
pub use title_repository::{in_memory::InMemoryTitleRepository, supabase::SupabaseTitleRepository};

pub use salary_repository::SalaryRepository;
pub use salary_repository::{
    in_memory::InMemorySalaryRepository, supabase::SupabaseSalaryRepository,
};
