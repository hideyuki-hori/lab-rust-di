INSERT INTO audit_logs (id, event_type, payload, created_at)
VALUES ($1, $2, $3, $4)
RETURNING id, event_type, payload, created_at
