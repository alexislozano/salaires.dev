#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug)]
pub enum Error {
    Empty,
}

impl TryFrom<String> for Company {
    type Error = Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        if raw.is_empty() {
            Err(Error::Empty)
        } else {
            Ok(Self { raw })
        }
    }
}
