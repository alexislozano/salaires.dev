#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Xp {
    raw: i32,
}

impl From<Xp> for i32 {
    fn from(xp: Xp) -> Self {
        xp.raw
    }
}

impl From<Xp> for String {
    fn from(xp: Xp) -> Self {
        format!("{}", xp.raw)
    }
}

#[derive(Clone)]
pub enum Error {
    Negative,
    NotANumber,
}

impl TryFrom<i32> for Xp {
    type Error = Error;

    fn try_from(raw: i32) -> Result<Self, Self::Error> {
        if raw < 0 {
            Err(Error::Negative)
        } else {
            Ok(Self { raw })
        }
    }
}

impl TryFrom<String> for Xp {
    type Error = Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        match raw.parse::<i32>() {
            Ok(raw) => Xp::try_from(raw),
            _ => Err(Error::NotANumber),
        }
    }
}
