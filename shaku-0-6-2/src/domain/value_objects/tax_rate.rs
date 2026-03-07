#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TaxRate(f64);

impl TaxRate {
    pub fn new(rate: f64) -> Result<Self, String> {
        if !(0.0..=1.0).contains(&rate) {
            return Err("TaxRate must be between 0.0 and 1.0".to_string());
        }
        Ok(Self(rate))
    }

    pub fn as_f64(self) -> f64 {
        self.0
    }
}
