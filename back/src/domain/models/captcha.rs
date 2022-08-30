pub struct Captcha {
    raw: String,
}

impl Captcha {
    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            raw: String::from("123456"),
        }
    }
}

impl From<Captcha> for String {
    fn from(captcha: Captcha) -> Self {
        captcha.raw
    }
}

impl TryFrom<String> for Captcha {
    type Error = ();

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        if raw.is_empty() {
            Err(())
        } else {
            Ok(Self { raw })
        }
    }
}
