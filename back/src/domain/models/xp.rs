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

impl TryFrom<i32> for Xp {
    type Error = ();

    fn try_from(raw: i32) -> Result<Self, Self::Error> {
        if raw < 0 {
            Err(())
        } else {
            Ok(Self { raw })
        }
    }
}
