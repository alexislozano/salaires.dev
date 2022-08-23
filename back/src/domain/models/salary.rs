use super::{Company, Compensation, Date, Level, Location, Stock, Title, Xp};

#[derive(Clone, Debug, PartialEq)]
pub struct Salary {
    pub company: Company,
    pub title: Title,
    pub location: Location,
    pub compensation: Compensation,
    pub date: Date,
    pub stock: Option<Stock>,
    pub level: Option<Level>,
    pub company_xp: Option<Xp>,
    pub total_xp: Option<Xp>,
}

impl Salary {
    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            company: Company::test(),
            title: Title::test(),
            location: Location::test(),
            compensation: Compensation::test(),
            date: Date::test(),
            stock: None,
            level: None,
            company_xp: None,
            total_xp: None,
        }
    }

    pub fn new(
        company: Company,
        title: Title,
        location: Location,
        compensation: Compensation,
        date: Date,
        stock: Option<Stock>,
        level: Option<Level>,
        company_xp: Option<Xp>,
        total_xp: Option<Xp>,
    ) -> Self {
        Self {
            company,
            title,
            location,
            compensation,
            date,
            stock,
            level,
            company_xp,
            total_xp,
        }
    }
}
