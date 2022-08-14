#[derive(Clone)]
pub struct Email {
    raw: String,
}

impl Email {
    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            raw: String::from("bonjour@salaires.dev"),
        }
    }
}

impl TryFrom<String> for Email {
    type Error = ();

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        if raw.contains("@") {
            Ok(Self { raw })
        } else {
            Err(())
        }
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.raw
    }
}
