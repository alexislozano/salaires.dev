mod salary;

pub use salary::{in_memory::InMemorySalaryRepository, supabase::SupabaseSalaryRepository};
pub use salary::{FetchAllError, InsertError, SalaryRepository};
