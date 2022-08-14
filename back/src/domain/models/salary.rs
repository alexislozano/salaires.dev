use super::{Company, Compensation, Date, Level, Location, Stock, Xp};

#[derive(Clone, Debug, PartialEq)]
pub struct Salary {
    pub company: Company,
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
