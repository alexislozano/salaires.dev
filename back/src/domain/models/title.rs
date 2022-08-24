#[derive(Clone, Debug, PartialEq)]
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

impl TryFrom<String> for Title {
    type Error = ();

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        if raw.is_empty() {
            Err(())
        } else {
            Ok(Self { raw })
        }
    }
}
