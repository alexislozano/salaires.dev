#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Title {
    raw: String,
}

impl Title {
    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            raw: String::from("Dev Fullstack"),
        }
    }
}

impl From<Title> for String {
    fn from(title: Title) -> Self {
        title.raw
    }
}

#[derive(Clone)]
pub enum Error {
    Empty,
}

impl TryFrom<String> for Title {
    type Error = Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        if raw.is_empty() {
            Err(Error::Empty)
        } else {
            Ok(Self { raw })
        }
    }
}
