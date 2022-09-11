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

impl TryFrom<String> for Email {
    type Error = ();

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        if raw.contains("@") {
            if FORBIDDEN_DOMAINS.contains(&raw.as_str()) {
                Err(())
            } else {
                Ok(Self { raw })
            }
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
