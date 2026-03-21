use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type,
)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct Quantity(pub(crate) i32);

impl Quantity {
    pub fn new(value: i32) -> Result<Self, String> {
        if value <= 0 {
            return Err(format!("Quantity must be positive, got {value}"));
        }
        Ok(Self(value))
    }

    pub fn as_negative_i32(&self) -> i32 {
        -self.0
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
