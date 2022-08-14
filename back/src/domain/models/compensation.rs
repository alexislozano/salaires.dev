#[derive(Clone, Debug, PartialEq)]
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

impl TryFrom<i32> for Compensation {
    type Error = ();

    fn try_from(raw: i32) -> Result<Self, Self::Error> {
        if raw < 0 {
            Err(())
        } else {
            Ok(Self { raw })
        }
    }
}
