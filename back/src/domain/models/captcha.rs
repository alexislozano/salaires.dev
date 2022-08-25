use captcha as captcha_lib;
use captcha_lib::Difficulty;

#[derive(Clone, PartialEq)]
pub struct Captcha {
    raw: String,
}

impl Captcha {
    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            raw: String::from("abc123"),
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
        if raw.len() > 0 {
            Ok(Self { raw })
        } else {
            Err(())
        }
    }
}

pub struct Challenge {
    raw: String,
}

impl From<Challenge> for String {
    fn from(challenge: Challenge) -> Self {
        challenge.raw
    }
}

pub fn generate() -> Result<(Captcha, Challenge), ()> {
    let captcha = captcha_lib::gen(Difficulty::Hard);
    let value = captcha.chars_as_string();
    let img = captcha.as_base64().ok_or(())?;
    Ok((Captcha { raw: value }, Challenge { raw: img }))
}
