#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Junior,
    Mid,
    Senior,
}

impl Level {
    pub fn all() -> Vec<Level> {
        vec![Self::Junior, Self::Mid, Self::Senior]
    }
}

impl From<Level> for String {
    fn from(level: Level) -> Self {
        match level {
            Level::Junior => "Junior",
            Level::Mid => "Mid",
            Level::Senior => "Senior",
        }
        .into()
    }
}

#[derive(Clone)]
pub enum Error {
    NotFound,
}

impl TryFrom<String> for Level {
    type Error = Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        match raw.as_str() {
            "Junior" => Ok(Self::Junior),
            "Mid" => Ok(Self::Mid),
            "Senior" => Ok(Self::Senior),
            _ => Err(Error::NotFound),
        }
    }
}
