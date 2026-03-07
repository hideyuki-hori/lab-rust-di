INSERT INTO orders (id, product_id, quantity, subtotal, tax_amount, shipping_fee, total_price, status, created_at)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
RETURNING *
