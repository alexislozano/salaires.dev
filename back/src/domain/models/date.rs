use chrono::NaiveDate;

#[derive(Clone, Debug, PartialEq)]
pub struct Date {
    raw: NaiveDate,
}

impl Date {
    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            raw: NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        }
    }
}

impl From<Date> for NaiveDate {
    fn from(date: Date) -> Self {
        date.raw
    }
}

impl From<NaiveDate> for Date {
    fn from(raw: NaiveDate) -> Self {
        Self { raw }
    }
}
