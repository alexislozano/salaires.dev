pub mod company;
pub mod location;
pub mod salary;

pub use company::CompanyRepository;
pub use company::{in_memory::InMemoryCompanyRepository, supabase::SupabaseCompanyRepository};
pub use location::LocationRepository;
pub use location::{in_memory::InMemoryLocationRepository, supabase::SupabaseLocationRepository};
pub use salary::SalaryRepository;
pub use salary::{in_memory::InMemorySalaryRepository, supabase::SupabaseSalaryRepository};
