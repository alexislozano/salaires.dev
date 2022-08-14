use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Token {
    raw: String,
}

impl Token {
    pub fn generate() -> Self {
        let mut rng = thread_rng();
        let mut raw = String::from("");

        for _ in 0..6 {
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
