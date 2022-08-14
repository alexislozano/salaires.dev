use crate::domain::models::Salary;
use crate::repositories::{InsertError, SalaryRepository};
use std::sync::Arc;

pub enum Error {
    Unknown,
}

pub fn insert_salary(repo: Arc<dyn SalaryRepository>, salary: Salary) -> Result<(), Error> {
    match repo.insert(salary) {
        Ok(_) => Ok(()),
        Err(InsertError::Unknown) => Err(Error::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemorySalaryRepository;

    #[test]
    fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let salary = Salary::test();
        let repo = Arc::new(InMemorySalaryRepository::new().with_error());

        let res = insert_salary(repo, salary);

        match res {
            Err(Error::Unknown) => {}
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_return_ok_otherwise() {
        let salary = Salary::test();
        let repo = Arc::new(InMemorySalaryRepository::new());

        let res = insert_salary(repo, salary);

        match res {
            Ok(()) => {}
            _ => unreachable!(),
        };
    }
}
