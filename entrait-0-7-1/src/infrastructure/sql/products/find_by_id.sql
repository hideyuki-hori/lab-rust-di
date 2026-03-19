SELECT id, name, price, stock, description, created_at, updated_at
FROM products
WHERE id = $1
