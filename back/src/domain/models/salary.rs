use std::cmp::Ordering;

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
    key: Key,
    direction: Direction
}

impl Order {
    pub fn new(key: Key, direction: Direction) -> Self {
        Self { key, direction}
    }
}

impl Default for Order {
    fn default() -> Self {
        Self { key: Key::Date, direction: Direction::Desc }
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

impl TryFrom<String> for Key {
    type Error = ();

    fn try_from(key: String) -> Result<Self, Self::Error> {
        match key.as_str() {
            "company" => Ok(Self::Company),
            "title" => Ok(Self::Title),
            "location" => Ok(Self::Location),
            "compensation" => Ok(Self::Compensation),
            "company_xp" => Ok(Self::CompanyXp),
            "total_xp" => Ok(Self::TotalXp),
            "level" => Ok(Self::Level),
            "date" => Ok(Self::Date),
            _ => Err(())
        }
    }
}

#[derive(Clone)]
pub enum Direction {
    Asc,
    Desc
}

impl TryFrom<String> for Direction {
    type Error = ();

    fn try_from(direction: String) -> Result<Self, Self::Error> {
        match direction.as_str() {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err(())
        }
    }
}