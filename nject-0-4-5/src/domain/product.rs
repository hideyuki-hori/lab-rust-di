use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::value_objects::{Price, ProductDescription, ProductId, ProductName, Quantity};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: ProductId,
    pub name: ProductName,
    pub price: Price,
    pub stock: Quantity,
    pub description: ProductDescription,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
