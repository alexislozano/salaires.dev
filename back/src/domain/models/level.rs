#[derive(Clone, Debug, PartialEq)]
pub enum Level {
    Junior,
    Mid,
    Senior,
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

impl TryFrom<String> for Level {
    type Error = ();

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        match raw.as_str() {
            "Junior" => Ok(Self::Junior),
            "Mid" => Ok(Self::Mid),
            "Senior" => Ok(Self::Senior),
            _ => Err(()),
        }
    }
}
