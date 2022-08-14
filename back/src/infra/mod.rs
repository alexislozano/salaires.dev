pub mod company_repository;
pub mod location_repository;
pub mod salary_repository;
pub mod token_repository;
pub mod token_sender;

pub use company_repository::CompanyRepository;
pub use company_repository::{
    in_memory::InMemoryCompanyRepository, supabase::SupabaseCompanyRepository,
};

pub use location_repository::LocationRepository;
pub use location_repository::{
    in_memory::InMemoryLocationRepository, supabase::SupabaseLocationRepository,
};

pub use salary_repository::SalaryRepository;
pub use salary_repository::{
    in_memory::InMemorySalaryRepository, supabase::SupabaseSalaryRepository,
};

pub use token_repository::TokenRepository;
pub use token_repository::{in_memory::InMemoryTokenRepository, supabase::SupabaseTokenRepository};

pub use token_sender::TokenSender;
pub use token_sender::{email::EmailTokenSender, stub::StubTokenSender};
