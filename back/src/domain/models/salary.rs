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
}
