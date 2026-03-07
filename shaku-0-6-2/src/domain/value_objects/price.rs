use std::ops::Mul;

use super::money::money_type;
use super::quantity::Quantity;
use super::subtotal::Subtotal;

money_type!(Price);

impl Price {
    pub fn new(value: i64) -> Result<Self, String> {
        if value < 0 {
            return Err("Price must be non-negative".to_string());
        }
        Ok(Self(value))
    }
}

impl Mul<Quantity> for Price {
    type Output = Subtotal;
    fn mul(self, rhs: Quantity) -> Subtotal {
        Subtotal(self.0 * rhs.0 as i64)
    }
}
