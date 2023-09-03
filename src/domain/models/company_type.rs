#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompanyType {
    GrandGroupe,
    PME,
    ESN
}

impl CompanyType {
    pub fn all() -> Vec<CompanyType> {
        vec![Self::GrandGroupe, Self::PME, Self::ESN]
    }
}

impl From<CompanyType> for String {
    fn from(company_type: CompanyType) -> Self {
        match company_type {
            CompanyType::GrandGroupe => "Grand groupe",
            CompanyType::PME => "PME",
            CompanyType::ESN => "ESN",
        }
        .into()
    }
}

#[derive(Clone)]
pub enum Error {
    NotFound,
}

impl TryFrom<String> for CompanyType {
    type Error = Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        match raw.as_str() {
            "Grand groupe" => Ok(Self::GrandGroupe),
            "PME" => Ok(Self::PME),
            "ESN" => Ok(Self::ESN),
            _ => Err(Error::NotFound),
        }
    }
}
