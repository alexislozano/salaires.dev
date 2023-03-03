use std::{cmp::Ordering, collections::HashMap};

use super::{Company, Compensation, Date, Email, Id, Level, Location, Status, Title, Xp};

#[derive(Clone, Debug, PartialEq)]
pub struct Salary {
    pub id: Id,
    pub email: Email,
    pub company: Company,
    pub title: Option<Title>,
    pub location: Location,
    pub compensation: Compensation,
    pub date: Date,
    pub level: Option<Level>,
    pub company_xp: Option<Xp>,
    pub total_xp: Option<Xp>,
    pub status: Status,
}

impl Salary {
    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            id: Id::generate(),
            email: Email::test(),
            company: Company::test(),
            title: None,
            location: Location::test(),
            compensation: Compensation::test(),
            date: Date::test(),
            level: None,
            company_xp: None,
            total_xp: None,
            status: Status::Waiting,
        }
    }

    pub fn new(
        id: Id,
        email: Email,
        company: Company,
        title: Option<Title>,
        location: Location,
        compensation: Compensation,
        date: Date,
        level: Option<Level>,
        company_xp: Option<Xp>,
        total_xp: Option<Xp>,
        status: Status,
    ) -> Self {
        Self {
            id,
            email,
            company,
            title,
            location,
            compensation,
            date,
            level,
            company_xp,
            total_xp,
            status,
        }
    }

    pub fn confirm(&mut self) {
        self.status = Status::Confirmed;
    }

    pub fn compare(a: &Salary, b: &Salary, order: &Order) -> Ordering {
        let ordering = match order.key {
            Key::Company => a.company.cmp(&b.company),
            Key::Title => a.title.cmp(&b.title),
            Key::Location => a.location.cmp(&b.location),
            Key::Compensation => a.compensation.cmp(&b.compensation),
            Key::CompanyXp => a.company_xp.cmp(&b.company_xp),
            Key::TotalXp => a.total_xp.cmp(&b.total_xp),
            Key::Level => a.level.cmp(&b.level),
            Key::Date => a.date.cmp(&b.date)
        };
        match order.direction {
            Direction::Asc => ordering,
            Direction::Desc => ordering.reverse()
        }
    }
}

#[derive(Clone)]
pub struct Order {
    pub key: Key,
    pub direction: Direction
}

impl Order {
    pub fn new(key: Key, direction: Direction) -> Self {
        Self { key, direction}
    }
}

impl Default for Order {
    fn default() -> Self {
        Self { key: Key::default(), direction: Direction::default() }
    }
}

impl From<HashMap<String, String>> for Order {
    fn from(hash: HashMap<String, String>) -> Self {
        let key = match hash.get("key") {
            Some(k) => Key::from(String::from(k)),
            None => Key::default()
        };
        let direction = match hash.get("direction") {
            Some(d) => Direction::from(String::from(d)),
            None => Direction::default()
        };
        Self::new(key, direction)
    }
}

#[derive(Clone)]
pub enum Key {
    Company,
    Title,
    Location,
    Compensation,
    CompanyXp,
    TotalXp,
    Level,
    Date,
}

impl Default for Key {
    fn default() -> Self {
        Self::Date
    }
}

impl From<String> for Key {
    fn from(key: String) -> Self {
        match key.as_str() {
            "company" => Self::Company,
            "title" => Self::Title,
            "location" => Self::Location,
            "compensation" => Self::Compensation,
            "company_xp" => Self::CompanyXp,
            "total_xp" => Self::TotalXp,
            "level" => Self::Level,
            "date" => Self::Date,
            _ => Self::default()
        }
    }
}

impl From<Key> for String {
    fn from(key: Key) -> Self {
        match key {
            Key::Company => "company",
            Key::Title => "title",
            Key::Location => "location",
            Key::Compensation => "compensation",
            Key::CompanyXp => "company_xp",
            Key::TotalXp => "total_xp",
            Key::Level => "level",
            Key::Date => "date",
        }.into()
    }
}

#[derive(Clone)]
pub enum Direction {
    Asc,
    Desc
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
            _ => Self::default()
        }
    }
}

impl From<Direction> for String {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Asc => "asc",
            Direction::Desc => "desc",
        }.into()
    }
}