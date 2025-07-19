-- Add migration script here

-- up
CREATE INDEX IF NOT EXISTS
  idx_request_ticket_opened_by_created_at
  ON request_ticket(opened_by, created_at DESC);

-- down
DROP INDEX IF EXISTS idx_request_ticket_wallet_created_at;
