-- Add migration script here

-- validators table
CREATE TABLE IF NOT EXISTS validators (
  id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  wallet_address  TEXT        NOT NULL UNIQUE,
  name            TEXT        NOT NULL,
  bio             TEXT,
  created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- programming languages master list
CREATE TABLE IF NOT EXISTS programming_languages (
  id   UUID   PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT   NOT NULL UNIQUE
);

-- expertise areas master list
CREATE TABLE IF NOT EXISTS expertise_areas (
  id   UUID   PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT   NOT NULL UNIQUE
);

-- many‑to‑many from validators → languages
CREATE TABLE IF NOT EXISTS validator_programming_languages (
  validator_id UUID NOT NULL REFERENCES validators(id) ON DELETE CASCADE,
  language_id  UUID NOT NULL REFERENCES programming_languages(id) ON DELETE CASCADE,
  PRIMARY KEY (validator_id, language_id)
);

-- many‑to‑many from validators → expertise
CREATE TABLE IF NOT EXISTS validator_expertise_areas (
  validator_id UUID NOT NULL REFERENCES validators(id) ON DELETE CASCADE,
  expertise_id UUID NOT NULL REFERENCES expertise_areas(id) ON DELETE CASCADE,
  PRIMARY KEY (validator_id, expertise_id)
);

-- index for fast wallet lookups
CREATE INDEX IF NOT EXISTS idx_validators_wallet
  ON validators(wallet_address);
