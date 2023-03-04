use super::{ConfirmError, FetchAllError, InsertError, SalaryRepository};
use crate::domain::models::{salary::Key, Id, Order, Salary, Status};
use async_trait::async_trait;
use axum::http::HeaderMap;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

pub struct SupabaseSalaryRepository {
    url: String,
    key: String,
}

impl SupabaseSalaryRepository {
    pub fn new() -> Self {
        Self {
            url: env::var("SUPABASE_URL").expect("SUPABASE_URL env var"),
            key: env::var("SUPABASE_KEY").expect("SUPABASE_KEY env var"),
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let api_key = self.key.parse().unwrap();
        let authorization = format!("Bearer {}", self.key).parse().unwrap();

        headers.insert("apikey", api_key);
        headers.insert("Authorization", authorization);

        headers
    }
}

#[async_trait]
impl SalaryRepository for SupabaseSalaryRepository {
    async fn confirm(&self, id: Id) -> Result<(), ConfirmError> {
        let client = reqwest::Client::new();
        match client
            .patch(format!("{}salaries?id=eq.{}", self.url, Uuid::from(id)))
            .headers(self.headers())
            .json(&SupabaseStatus::from(Status::Confirmed))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            _ => Err(ConfirmError::Unknown("could not send request")),
        }
    }

    async fn fetch_all(&self, order: Order<Key>) -> Result<Vec<Salary>, FetchAllError> {
        let client = reqwest::Client::new();
        let res = match client
            .get(format!(
                "{}salaries?select=*&status=eq.{}&order={}.{}",
                self.url,
                String::from(Status::Published),
                String::from(order.key),
                String::from(order.direction)
            ))
            .headers(self.headers())
            .send()
            .await
        {
            Ok(res) => res,
            _ => return Err(FetchAllError::Unknown("could not send request")),
        };

        let supabase_salaries = match res.json::<Vec<SupabaseSalary>>().await {
            Ok(salaries) => salaries,
            _ => return Err(FetchAllError::Unknown("could not parse json")),
        };

        let mut salaries = vec![];

        for supabase_salary in supabase_salaries.into_iter() {
            match supabase_salary.try_into() {
                Ok(salary) => salaries.push(salary),
                _ => return Err(FetchAllError::Unknown("could not convert to domain")),
            }
        }

        Ok(salaries)
    }

    async fn insert(&self, salary: Salary) -> Result<(), InsertError> {
        let client = reqwest::Client::new();
        match client
            .post(format!("{}salaries", self.url))
            .headers(self.headers())
            .json(&SupabaseSalary::from(salary))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            _ => Err(InsertError::Unknown("could not send request")),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SupabaseSalary {
    id: Uuid,
    email: String,
    company: String,
    title: Option<String>,
    location: String,
    compensation: i32,
    date: NaiveDate,
    level: Option<String>,
    company_xp: Option<i32>,
    total_xp: Option<i32>,
    status: String,
}

impl From<Salary> for SupabaseSalary {
    fn from(salary: Salary) -> Self {
        Self {
            id: salary.id.into(),
            email: salary.email.into(),
            company: salary.company.into(),
            title: salary.title.map(|title| title.into()),
            location: salary.location.into(),
            compensation: salary.compensation.into(),
            date: salary.date.into(),
            level: salary.level.map(|level| level.into()),
            company_xp: salary.company_xp.map(|company_xp| company_xp.into()),
            total_xp: salary.total_xp.map(|total_xp| total_xp.into()),
            status: salary.status.into(),
        }
    }
}

impl TryFrom<SupabaseSalary> for Salary {
    type Error = ();

    fn try_from(salary: SupabaseSalary) -> Result<Self, Self::Error> {
        Ok(Self::new(
            salary.id.into(),
            salary.email.try_into()?,
            salary.company.try_into()?,
            if let Some(raw) = salary.title {
                Some(raw.try_into()?)
            } else {
                None
            },
            salary.location.try_into()?,
            salary.compensation.try_into()?,
            salary.date.into(),
            if let Some(raw) = salary.level {
                Some(raw.try_into()?)
            } else {
                None
            },
            if let Some(raw) = salary.company_xp {
                Some(raw.try_into()?)
            } else {
                None
            },
            if let Some(raw) = salary.total_xp {
                Some(raw.try_into()?)
            } else {
                None
            },
            salary.status.try_into()?,
        ))
    }
}

#[derive(Serialize)]
struct SupabaseStatus {
    status: String,
}

impl From<Status> for SupabaseStatus {
    fn from(status: Status) -> Self {
        Self {
            status: status.into(),
        }
    }
}
