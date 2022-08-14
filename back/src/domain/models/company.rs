#[derive(Clone, Debug, PartialEq)]
pub struct Company {
    raw: String,
}

impl Company {
    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            raw: String::from("Google"),
        }
    }
}

impl From<Company> for String {
    fn from(company: Company) -> Self {
        company.raw
    }
}

impl TryFrom<String> for Company {
    type Error = ();

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        if raw.is_empty() {
            Err(())
        } else {
            Ok(Self { raw })
        }
    }
}
