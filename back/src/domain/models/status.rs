#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Waiting,
    Confirmed,
    Published,
}

impl From<Status> for String {
    fn from(status: Status) -> Self {
        match status {
            Status::Waiting => "WAITING",
            Status::Confirmed => "CONFIRMED",
            Status::Published => "PUBLISHED",
        }
        .into()
    }
}

impl TryFrom<String> for Status {
    type Error = ();

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        match raw.as_str() {
            "WAITING" => Ok(Self::Waiting),
            "CONFIRMED" => Ok(Self::Confirmed),
            "PUBLISHED" => Ok(Self::Published),
            _ => Err(()),
        }
    }
}
