use std::collections::HashMap;

#[derive(Clone)]
pub struct Order<K> {
    pub key: K,
    pub direction: Direction,
}

impl<K> Order<K> {
    pub fn new(key: K, direction: Direction) -> Self {
        Self { key, direction }
    }
}

impl<K> Default for Order<K>
where
    K: Default,
{
    fn default() -> Self {
        Self {
            key: K::default(),
            direction: Direction::default(),
        }
    }
}

impl<K> From<HashMap<String, String>> for Order<K>
where
    K: Default + From<String>,
{
    fn from(hash: HashMap<String, String>) -> Self {
        let key = match hash.get("key") {
            Some(k) => K::from(String::from(k)),
            None => K::default(),
        };
        let direction = match hash.get("direction") {
            Some(d) => Direction::from(String::from(d)),
            None => Direction::default(),
        };
        Self::new(key, direction)
    }
}

#[derive(Clone)]
pub enum Direction {
    Asc,
    Desc,
}

impl Direction {
    pub fn reverse(&self) -> Self {
        match self {
            Self::Asc => Self::Desc,
            Self::Desc => Self::Asc,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::Desc
    }
}

impl From<String> for Direction {
    fn from(direction: String) -> Self {
        match direction.as_str() {
            "asc" => Self::Asc,
            "desc" => Self::Desc,
            _ => Self::default(),
        }
    }
}

impl From<Direction> for String {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Asc => "asc",
            Direction::Desc => "desc",
        }
        .into()
    }
}
