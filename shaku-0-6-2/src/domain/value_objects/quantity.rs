use std::fmt;
use std::ops::Neg;

use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type,
)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct Quantity(pub i32);

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<i32> for Quantity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Neg for Quantity {
    type Output = Self;
    fn neg(self) -> Self {
        Quantity(-self.0)
    }
}
