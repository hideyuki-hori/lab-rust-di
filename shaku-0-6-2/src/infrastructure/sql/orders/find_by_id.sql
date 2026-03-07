SELECT id, product_id, quantity, subtotal, tax_amount, shipping_fee, total_price, status, created_at
FROM orders
WHERE id = $1
