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

impl TryFrom<String> for Location {
    type Error = ();

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        if raw.is_empty() {
            Err(())
        } else {
            Ok(Self { raw })
        }
    }
}
