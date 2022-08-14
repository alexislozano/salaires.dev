mod salary;

pub use salary::in_memory::InMemorySalaryRepository;
pub use salary::{FetchAllError, InsertError, SalaryRepository};
