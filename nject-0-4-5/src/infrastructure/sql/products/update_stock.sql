UPDATE products
SET stock = stock + $1, updated_at = NOW()
WHERE id = $2 AND stock + $1 >= 0
