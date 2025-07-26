CREATE INDEX IF NOT EXISTS idx_request_ticket_opened_by_created_at
    ON request_ticket (opened_by, created_at DESC);
