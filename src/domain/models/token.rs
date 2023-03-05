use rand::{thread_rng, Rng};

const LENGTH: usize = 64;

#[derive(Clone, PartialEq)]
pub struct Token {
    raw: String,
}

impl Token {
    pub fn generate() -> Self {
        let mut rng = thread_rng();
        let mut raw = String::from("");

        for _ in 0..LENGTH {
            raw.push_str(&format!("{}", rng.gen_range(0..10)));
        }

        Self { raw }
    }
}

impl From<Token> for String {
    fn from(token: Token) -> Self {
        token.raw
    }
}

impl TryFrom<String> for Token {
    type Error = ();

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        if raw.len() == LENGTH {
            Ok(Self { raw })
        } else {
            Err(())
        }
    }
}
