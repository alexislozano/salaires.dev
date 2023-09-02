#[derive(Clone, Debug, PartialEq)]
pub enum CompanyType {
    GrandGroupe,
    PME,
    ESN
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

impl TryFrom<String> for CompanyType {
    type Error = ();

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        match raw.as_str() {
            "Grand groupe" => Ok(Self::GrandGroupe),
            "PME" => Ok(Self::PME),
            "ESN" => Ok(Self::ESN),
            _ => Err(()),
        }
    }
}
