SELECT id, event_type, payload, created_at
FROM audit_logs
ORDER BY created_at DESC
