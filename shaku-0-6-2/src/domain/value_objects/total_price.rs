use super::money::money_type;
use super::shipping_fee::ShippingFee;
use super::subtotal::Subtotal;
use super::tax_amount::TaxAmount;

money_type!(TotalPrice);

impl TotalPrice {
    pub fn new(subtotal: Subtotal, tax_amount: TaxAmount, shipping_fee: ShippingFee) -> Self {
        Self(subtotal.0 + tax_amount.0 + shipping_fee.0)
    }
}
