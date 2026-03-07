use super::money::money_type;
use super::tax_amount::TaxAmount;
use super::tax_rate::TaxRate;

money_type!(Subtotal);

impl Subtotal {
    pub fn apply_rate(self, rate: TaxRate) -> TaxAmount {
        TaxAmount((self.0 as f64 * rate.as_f64()).round() as i64)
    }
}
