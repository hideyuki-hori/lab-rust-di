INSERT INTO products (id, name, price, stock, description, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING *
