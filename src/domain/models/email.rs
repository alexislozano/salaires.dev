const FORBIDDEN_DOMAINS: [&str; 13] = [
    "gmail",
    "yahoo",
    "hotmail",
    "aol",
    "wanadoo",
    "msn",
    "live",
    "free",
    "outlook",
    "laposte",
    "protonmail",
    "yopmail",
    "minutemail",
];

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug)]
pub enum Error {
    NotContainingAnAt,
    NotPro,
}

impl TryFrom<String> for Email {
    type Error = Error;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        if raw.contains("@") {
            if FORBIDDEN_DOMAINS
                .iter()
                .any(|domain| raw.as_str().contains(domain))
            {
                Err(Error::NotPro)
            } else {
                Ok(Self { raw })
            }
        } else {
            Err(Error::NotContainingAnAt)
        }
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.raw
    }
}
