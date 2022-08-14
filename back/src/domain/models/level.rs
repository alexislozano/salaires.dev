#[derive(Clone, Debug, PartialEq)]
pub struct Level {
    raw: String,
}

impl From<Level> for String {
    fn from(level: Level) -> Self {
        level.raw
    }
}

impl TryFrom<String> for Level {
    type Error = ();

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        if raw.is_empty() {
            Err(())
        } else {
            Ok(Self { raw })
        }
    }
}
