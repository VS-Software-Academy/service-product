use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub id: Uuid,
    pub description: String,
    pub created_at: NaiveDateTime,
}
