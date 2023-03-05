use std::cmp::Ordering;

use super::{
    Company, Compensation, Date, Direction, Email, Id, Level, Location, Order, Status, Title, Xp,
};

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

    pub fn compare(a: &Salary, b: &Salary, order: &Order<Key>) -> Ordering {
        let ordering = match order.key {
            Key::Company => a.company.cmp(&b.company),
            Key::Title => a.title.cmp(&b.title),
            Key::Location => a.location.cmp(&b.location),
            Key::Compensation => a.compensation.cmp(&b.compensation),
            Key::CompanyXp => a.company_xp.cmp(&b.company_xp),
            Key::TotalXp => a.total_xp.cmp(&b.total_xp),
            Key::Level => a.level.cmp(&b.level),
            Key::Date => a.date.cmp(&b.date),
        };
        match order.direction {
            Direction::Asc => ordering,
            Direction::Desc => ordering.reverse(),
        }
    }
}

#[derive(Clone, PartialEq)]
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
            _ => Self::default(),
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
        }
        .into()
    }
}
