use super::money::money_type;
use super::shipping_fee::ShippingFee;
use super::subtotal::Subtotal;
use super::tax_amount::TaxAmount;

money_type!(TotalPrice);

impl TotalPrice {
    pub fn calculate(
        subtotal: Subtotal,
        tax_amount: TaxAmount,
        shipping_fee: ShippingFee,
    ) -> Result<Self, String> {
        subtotal
            .0
            .checked_add(tax_amount.0)
            .and_then(|v| v.checked_add(shipping_fee.0))
            .map(Self)
            .ok_or_else(|| "TotalPrice overflow".to_string())
    }
}
