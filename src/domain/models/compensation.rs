#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Compensation {
    raw: i32,
}

impl Compensation {
    #[cfg(test)]
    pub fn test() -> Self {
        Self { raw: 100_000 }
    }
}

impl From<Compensation> for i32 {
    fn from(compensation: Compensation) -> Self {
        compensation.raw
    }
}

impl From<Compensation> for String {
    fn from(compensation: Compensation) -> Self {
        format!("{}K", compensation.raw / 1000)
    }
}

#[derive(Clone)]
pub enum Error {
    Negative,
    NotANumber,
}

impl TryFrom<i32> for Compensation {
    type Error = Error;

    fn try_from(raw: i32) -> Result<Self, Self::Error> {
        if raw < 0 {
            Err(Error::Negative)
        } else {
            Ok(Self { raw })
        }
    }
}

impl TryFrom<String> for Compensation {
    type Error = Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        match raw.parse::<i32>() {
            Ok(raw) => Compensation::try_from(raw),
            _ => Err(Error::NotANumber),
        }
    }
}
