#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    raw: String,
}

impl Location {
    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            raw: String::from("Paris"),
        }
    }
}

impl From<Location> for String {
    fn from(location: Location) -> Self {
        location.raw
    }
}

pub enum Error {
    Empty,
}

impl TryFrom<String> for Location {
    type Error = Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        if raw.is_empty() {
            Err(Error::Empty)
        } else {
            Ok(Self { raw })
        }
    }
}
