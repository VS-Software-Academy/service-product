use chrono::{NaiveDateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Product {
    pub id: Uuid,
    pub description: String,
    pub category: Uuid,
    pub price: Decimal,
    pub created_at: NaiveDateTime,
}

impl Product {
    pub fn new(description: String, category: Uuid, price: Decimal) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            category,
            price,
            created_at: Utc::now().naive_utc(),
        }
    }
}
