use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Id {
    raw: Uuid,
}

impl Id {
    pub fn generate() -> Self {
        Self {
            raw: Uuid::new_v4(),
        }
    }
}

impl From<Id> for Uuid {
    fn from(id: Id) -> Self {
        id.raw
    }
}

impl From<Uuid> for Id {
    fn from(raw: Uuid) -> Self {
        Self { raw }
    }
}
