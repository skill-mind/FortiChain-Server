-- Add migration script here
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_type') THEN
        CREATE TYPE user_type AS ENUM ('admin', 'user', 'support_agent');
    END IF;
END$$;

-- 2. Add the 'type' column to escrow_users
ALTER TABLE escrow_users
    ADD COLUMN IF NOT EXISTS type user_type NOT NULL DEFAULT 'user';

-- 3. Make assigned_to nullable in request_ticket
ALTER TABLE request_ticket
    ALTER COLUMN assigned_to DROP NOT NULL;