use crate::domain::models::Company;
use crate::repositories::{company::FetchAllError, CompanyRepository};
use std::sync::Arc;

pub enum Error {
    Unknown(&'static str),
}

pub async fn fetch_companies(repo: Arc<dyn CompanyRepository>) -> Result<Vec<Company>, Error> {
    match repo.fetch_all().await {
        Ok(companies) => Ok(companies),
        Err(FetchAllError::Unknown(str)) => Err(Error::Unknown(str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryCompanyRepository;

    #[tokio::test]
    async fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
        let repo = Arc::new(InMemoryCompanyRepository::new().with_error());

        let res = fetch_companies(repo).await;

        match res {
            Err(Error::Unknown(_)) => {}
            _ => unreachable!(),
        };
    }

    #[tokio::test]
    async fn it_should_return_all_companies_otherwise() {
        let company = Company::test();
        let repo = Arc::new(InMemoryCompanyRepository::new());
        repo.insert(company.clone());

        let res = fetch_companies(repo).await;

        match res {
            Ok(companies) => {
                assert_eq!(vec![company], companies);
            }
            _ => unreachable!(),
        };
    }
}
