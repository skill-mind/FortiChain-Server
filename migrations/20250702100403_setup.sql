-- Projects
-- A project can be uploaded on the platform for researchers and validators to conduct audits,
-- this project when created has to be verified that whoever brings it on the platform is the rightful owner,
-- afterwards, they can proceed to allocate a bounty for the project.
create table projects (
    id uuid primary key default uuid_generate_v1mc(),

    -- Project Metadata
    owner_address varchar(66) not null check (owner_address ~ '0x[a-fA-F0-9]{64}$'),
    contract_address varchar(66) not null unique check (contract_address ~ '0x[a-fA-F0-9]{64}$'),
    name varchar(256) not null check (length(name) >= 3),
    description text not null check (length(description) between 10 and 500),
    contact_info varchar(255) not null check (contact_info ~ '^[^@\s]+@[^@\s]+\.[^@\s]+$|^https?://.+$'),
    supporting_document_path text check (supporting_document_path ~ '^(https?|ftp)://[^\s/$.?#].[^\s]*$'),
    project_logo_path text check (project_logo_path ~ '^(https?|ftp)://[^\s/$.?#].[^\s]*$'),

    -- Governance
    is_verified boolean not null default false,
    verification_date timestamptz,
    repository_url text check (repository_url ~ '^(https?|ftp)://[^\s/$.?#].[^\s]*$'),
    constraint ck_projects_verification check (
        not is_verified or (is_verified and verification_date is not null and repository_url is not null)
    ),

    -- Bounty
    bounty_amount numeric(20, 2),
    bounty_currency varchar(10),
    bounty_expiry_date timestamptz,
    constraint ck_projects_bounty check (
        (bounty_amount is null and bounty_currency is null and bounty_expiry_date is null) or
        (bounty_amount is not null and bounty_currency is not null and bounty_expiry_date is not null)
    ),

    -- Timestamps
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    closed_at timestamptz
);

-- Comments for clarity and validation rules
comment on column projects.id is 'Unique identifier for the project.';
comment on column projects.owner_address is 'The wallet address of the user who owns this project.';
comment on column projects.description is 'Must be between 10-500 characters';
comment on column projects.contract_address is 'Expected format: Starknet Contract Address (0x + 64 hex chars)';
comment on column projects.supporting_document_path is 'PDF files only, max 5MB';
comment on column projects.project_logo_path is '500x500px PNG/JPG, max 3MB';
comment on column projects.contact_info is 'Email or social media link';
comment on column projects.is_verified is 'True if project ownership has been verified.';
comment on column projects.verification_date is 'Timestamp when project ownership was verified.';
comment on column projects.repository_url is 'Link to the public repository used for ownership verification.';
comment on column projects.bounty_amount is 'The amount of bounty allocated to the project.';
comment on column projects.bounty_currency is 'The currency type of the allocated bounty (e.g., STRK, USD).';
comment on column projects.bounty_expiry_date is 'The date by which the bounty must be claimed or completed.';
comment on column projects.closed_at IS 'Timestamp when the project was officially closed by its owner. NULL if the project is active.';

create table tags (
    id serial primary key,
    name varchar(50) not null unique check (name <> '')
);

create table project_tags (
    project_id uuid references projects(id) on delete cascade,
    tag_id integer references tags(id) on delete cascade,
    primary key (project_id, tag_id)
);


-- Escrows
-- A project owner can deposit to the platform's escrow account, from this escrow account, the project owner can decide
-- to allocate a bounty for the project(s) they create, from their balance on the escrow wallet. A project owner has the
-- liberty to withdraw from their available balance in the escrow account.
--
-- Say an entity deposits $5,000 to the platform's escrow, and then proceeds to allocate $3,000 bounty to their project on the
-- platform, their project now has $3,000 allocated as bounty and their escrow wallet balance is now $2,000; they proceed to
-- withdraw $1,500 from their escrow balance to their own wallet.
create table escrow_users (
    wallet_address varchar(66) primary key not null check (wallet_address ~ '0x[a-fA-F0-9]{64}$'),
    balance numeric(30, 2) not null default 0.0 check (balance >= 0),
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

comment on column escrow_users.wallet_address is 'The user''s Starknet wallet address. Used for deposits and withdrawals.';
comment on column escrow_users.balance is 'The user''s current available balance on the platform''s escrow.';

create type transaction_type as enum ('deposit', 'bounty_allocation', 'bounty_disbursement', 'withdrawal');
create type transaction_status as enum ('pending', 'completed', 'failed');

create table escrow_transactions (
    id uuid primary key default uuid_generate_v1mc(),
    wallet_address varchar(66) not null references escrow_users(wallet_address) on delete restrict,
    project_id uuid references projects(id) on delete restrict,
    type transaction_type not null,
    amount numeric(30, 2) not null check (amount > 0),
    currency varchar(10) not null,
    transaction_hash varchar(100) unique not null,
    status transaction_status not null default 'pending',
    notes text,
    created_at timestamptz not null default now(),
    updated_at timestamptz,

    constraint ck_escrow_transactions_project_required_check check (
        (type in ('bounty_allocation', 'bounty_disbursement') and project_id is not null) or
        (type in ('deposit', 'withdrawal') and project_id is null)
    )
);

comment on column escrow_transactions.id is 'Unique identifier for the transaction.';
comment on column escrow_transactions.wallet_address is 'The user associated with this transaction.';
comment on column escrow_transactions.project_id is 'The project associated with this transaction (e.g., for bounty allocations).';
comment on column escrow_transactions.type is 'Type of transaction: deposit, bounty_allocation, or withdrawal.';
comment on column escrow_transactions.amount is 'The amount of currency transacted (always positive).';
comment on column escrow_transactions.currency is 'The currency of the transaction (e.g., STRK, USD).';
comment on column escrow_transactions.transaction_hash is 'Hash of the corresponding blockchain transaction (if applicable).';
comment on column escrow_transactions.status is 'Current status of the transaction: pending, completed, or failed.';
comment on column escrow_transactions.notes is 'Additional notes for the transaction.';
comment on column escrow_transactions.created_at is 'Timestamp when the transaction record was created.';
comment on column escrow_transactions.updated_at is 'Timestamp when the transaction moved to a "completed" or "failed" status.';


-- Validator Profile
-- A qualified, verified professional can register on the platform to become a validator,
-- becoming a validator gives the ability to vet researchers submissions, if they are first,
-- valid findings, and if they are, the severity  of the finding.
-- severity ranges from low, medium, high to critical.
create domain public.phone as text check(
    length(value) between 10 and 19 and value ~ '^\+\d{1,3}\d{7,15}$'
);

comment on domain public.phone is 'Phone number in the international format E.164';

create type profile_verification as enum ('pending', 'verified', 'rejected');
create type document_type as enum ('id', 'passport', 'driving_license', 'voter''s_card');

create table validator_profiles (
    id uuid primary key not null default uuid_generate_v1mc(),
    wallet_address varchar(66) not null check (wallet_address ~ '0x[a-fA-F0-9]{64}$'),

    -- PII (Personal Identifiable Information)
    government_name varchar(256) not null check (length(government_name) > 3),
    date_of_birth date not null check (date_of_birth < CURRENT_DATE),
    nationality varchar(255) not null check (length(nationality) >= 3),
    email_address varchar(255) unique not null check (length(email_address) >= 5),
    mobile_number public.phone,

    -- Professional Background
    years_of_experience smallint not null check (years_of_experience >= 0),
    portfolio_website text check (length(portfolio_website) <= 500 and (portfolio_website is null or portfolio_website ~* '^https?://')),
    github_profile text check (length(github_profile) <= 500 and github_profile ~* '^https?://'),
    linkedin_profile text check (length(linkedin_profile) <= 500 and (linkedin_profile is null or linkedin_profile ~ '^https://(?:www\.)?linkedin\.com/.+')),
    resume_path text not null check (length(resume_path) <= 500),
    certifications_path text check (length(certifications_path) <= 500),

    -- KYC
    country varchar(50) not null check (length(country) >= 4),
    document document_type not null,
    document_front_path text not null check (length(document_front_path) <= 500),
    document_back_path text not null check (length(document_back_path) <= 500),
    verification profile_verification default 'pending',

    -- Time related
    created_at timestamptz default now(),
    updated_at timestamptz,

    constraint ck_validator_profile_age check (
        extract(year from age(date_of_birth)) >= 18
    )
);

create table programming_languages (
    id serial primary key,
    name varchar(255) not null check (name <> '')
);

create table validator_programming_languages (
    validator_id uuid references validator_profiles(id) on delete cascade,
    language_id integer references programming_languages(id) on delete cascade,
    primary key (validator_id, language_id)
);

create table expertise (
    id serial primary key,
    name varchar(255) not null check (name <> '')
);

create table validator_expertise (
    validator_id uuid references validator_profiles(id) on delete cascade,
    expertise_id integer references expertise(id) on delete cascade,
    primary key (validator_id, expertise_id)
);


-- Researcher
-- Researchers can write a report on any findings they make while auditing a project,
-- after a report has been submitted, the validator proceeds to access it and scores the
-- severity of the report made. if the report is unclear enough for the validator to make a decision,
-- they can ask for more information from the researcher.
-- if the report is rejected, the validator can provide a reason for rejection.
create type severity_level as enum ('low', 'medium', 'high', 'critical');
create type rejection_reason as enum ('duplicate_report', 'incomplete_information', 'already_known', 'out_of_scope');
create type report_status_type as enum (
    'submitted',        -- Initial state by researcher
    'assigned',         -- Assigned to a validator
    'in_review',        -- Validator is actively reviewing
    'info_requested',   -- Validator requested more info from researcher
    'info_provided',    -- Researcher provided requested info
    'accepted',         -- Validator accepted the report (severity assigned)
    'rejected',         -- Validator rejected the report (reason provided)
    'closed'            -- Report lifecycle is complete (e.g., bounty paid, or final rejection)
);

create table research_report (
    id uuid primary key default uuid_generate_v1mc(),

    -- Metadata
    title varchar(256) not null check (length(title) >= 3),
    project_id uuid not null references projects(id) on delete restrict,
    body text not null check (length(body) between 50 and 10000),

    -- Participants
    reported_by VARCHAR(66) NOT NULL CHECK (reported_by ~ '^0x[a-fA-F0-9]{64}$'),
    validated_by VARCHAR(66) CHECK (validated_by ~ '^0x[a-fA-F0-9]{64}$'),

    -- Report Assessment
    status report_status_type NOT NULL DEFAULT 'submitted',
    severity severity_level,
    allocated_reward NUMERIC(30, 2) CHECK (allocated_reward >= 0),

    -- Rejection/Information related
    reason rejection_reason,
    validator_notes text check (length(validator_notes) <= 1000),
    researcher_response text check (length(researcher_response) <= 1000),

    -- Timestamps
    created_at timestamptz default now(),
    updated_at timestamptz,

    constraint ck_research_report_reward check (
        (status in ('accepted') and severity is not null and validated_by is not null) or
        (status in ('rejected') and reason is not null and validated_by is not null) or
        (status not in ('accepted', 'rejected'))
    ),
    constraint ck_research_award check (
        (status = 'accepted' and severity is not null) or allocated_reward is null
    )
);


-- Help Center
-- Users can submit a request ticket to the platform's support. these tickets would then be assigned to support, who proceeds
-- to resolve the issue.
CREATE TYPE ticket_status_type AS ENUM (
    'open',             -- Initial state when submitted by user
    'assigned',         -- Assigned to a support agent
    'in_progress',      -- Agent is actively working on it
    'awaiting_user',    -- Agent is waiting for user's response/info
    'resolved',         -- Agent has provided a resolution
    'closed',           -- Ticket is formally closed (e.g., by user or after a period of inactivity)
    'reopened'          -- User reopens a resolved/closed ticket
);

create table request_ticket (
    id uuid primary key default uuid_generate_v1mc(),

    -- Ticket Information
    subject varchar(256) not null check (length(subject) >= 5),
    message text not null check (length(message) <= 5000),
    document_path text,
    opened_by varchar(66) not null check (opened_by ~ '0x[a-fA-F0-9]{64}$'),

    -- Ticket Status and Resolution
    status ticket_status_type not null default 'open',
    assigned_to varchar(66) not null check (assigned_to ~ '0x[a-fA-F0-9]{64}$'),
    response_subject varchar(50) not null check (response_subject <> ''),
    resolution_response text check (length(resolution_response) <= 5000),
    resolved boolean default false,

    -- Timestamps
    created_at timestamptz not null default now(),
    resolved_at timestamptz,
    updated_at timestamptz
);

-- Sample Queries
-- Default UPPERCASE SQL syntax is used here as syntax highlighting is turned off
-- due to these being commented out.


-- Projects Section
--
-- Sample 1: Create a project with tags
--
-- BEGIN;

-- WITH new_project AS (
--   INSERT INTO projects (
--     owner_address,
--     contract_address,
--     name,
--     description,
--     contact_info
--   ) VALUES (
--     '0x01a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b',
--     '0x02a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2c',
--     'StarkNet Yield Aggregator',
--     'A decentralized protocol for yield farming on the StarkNet ecosystem.',
--     'contact@starkyield.com'
--   ) RETURNING id
-- ),
-- upserted_tags AS (
--   INSERT INTO tags (name) VALUES
--   ('DeFi'),
--   ('Yield Farming')
--   ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
--   RETURNING id, name
-- )
-- INSERT INTO project_tags (project_id, tag_id)
-- SELECT np.id, ut.id
-- FROM new_project np, upserted_tags ut;

-- COMMIT;

-- Sample 2: Update a project to verified
--
-- UPDATE projects
-- SET
--   is_verified = TRUE,
--   verification_date = NOW(),
--   repository_url = 'https://github.com/stark-yield/protocol'
-- WHERE
--   contract_address = '0x02a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2c';

-- Sample 3: Allocate bounty to a project (Note: This is now handled via the escrow system)
--
-- BEGIN;
--
-- UPDATE projects
-- SET
--  bounty_amount = 1500,
--  bounty_currency = 'STRK',
--  bounty_expiry_date = '2025-12-31 23:59:59+00',
-- WHERE
--  contract_address = '0x246';
-- COMMIT;

-- Sample 4: Close a project
-- UPDATE projects
-- SET closed_at = NOW()
-- WHERE contract_address = '0x02a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2c';


-- Escrows Section
--
-- Sample 1: Deposit funds to Escrow
-- BEGIN;
--
-- First, ensure the user exists. If not, create them.
-- INSERT INTO escrow_users (wallet_address)
-- VALUES ('0x01a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b')
-- ON CONFLICT (wallet_address) DO NOTHING;
--
-- Record the transaction
-- INSERT INTO escrow_transactions (wallet_address, type, amount, currency, transaction_hash, status)
-- VALUES ('0x01a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b', 'deposit', 5000.00, 'STRK', '0xabc...def', 'completed');
--
-- Update the user's balance
-- UPDATE escrow_users
-- SET balance = balance + 5000.00
-- WHERE wallet_address = '0x01a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b';
--
-- COMMIT;
--
-- Sample 2: Allocate Bounty from Escrow to a Project
-- BEGIN;
--
-- -- This transaction would be atomic in a real application.
-- -- 1. Decrease user's escrow balance
-- UPDATE escrow_users
-- SET balance = balance - 3000.00
-- WHERE wallet_address = '0x01a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b'
--   AND balance >= 3000.00;
--
-- -- 2. Update the project's bounty details
-- UPDATE projects
-- SET
--   bounty_amount = bounty_amount + 3000.00,
--   bounty_currency = 'STRK',
--   bounty_expiry_date = '2025-12-31 23:59:59+00'
-- WHERE
--   contract_address = '0x02a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2c';
--
-- 3. Log the bounty allocation transaction
-- INSERT INTO escrow_transactions (wallet_address, project_id, type, amount, currency, transaction_hash, status)
-- SELECT
--     '0x01a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b',
--     p.id,
--     'bounty_allocation',
--     3000.00,
--     'STRK',
--     '0xghi...jkl', -- Internal or off-chain transaction reference
--     'completed'
-- FROM projects p
-- WHERE p.contract_address = '0x02a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2c';
--
-- COMMIT;

-- 3. Sample query to Withdraw funds from Escrow
-- declare
--     _withdraw_amount numeric := 1500.00;
--     _withdraw_currency varchar(10) := 'STRK';
--     _withdraw_tx_hash varchar(100) := '0x864';
-- begin
--     if (select balance from escrow_users where wallet_address = _test_wallet_address) >= _withdraw_amount then
--         insert into escrow_transactions (wallet_address, type, amount, currency, transaction_hash, status, notes)
--         values (_test_wallet_address, 'withdrawal', _withdraw_amount, _withdraw_currency, _withdraw_tx_hash, 'completed', 'Funds withdrawn to user wallet');

--         update escrow_users
--         set balance = balance - _withdraw_amount
--         where wallet_address = _test_wallet_address;

--         raise notice 'Withdrew % % from user %''s escrow.', _withdraw_amount, _withdraw_currency, _test_wallet_address;
--     else
--         raise notice 'Insufficient escrow balance for withdrawal.';
--     end if;
-- end;


-- Validators Section
--
-- Sample 1: Create a new Validator Profile
-- BEGIN;
--
-- WITH new_validator AS (
--   INSERT INTO validator_profiles (
--     wallet_address, government_name, date_of_birth, nationality, email_address, mobile_number,
--     years_of_experience, portfolio_website, github_profile, linkedin_profile, resume_path,
--     country, document, document_front_path
--   ) VALUES (
--     '0x03a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2d', 'Jane Doe', '1990-05-15', 'Nigerian', 'jane.doe@example.com', '+2348123456789',
--     5, 'https://janedoe.dev', 'https://github.com/janedoe', 'https://linkedin.com/in/janedoe', '/resumes/jane_doe.pdf',
--     'Nigeria', 'passport', '/kyc/jane_doe_passport.jpg'
--   ) RETURNING id
-- ),
-- new_langs AS (
--   INSERT INTO programming_languages (name) VALUES ('Cairo'), ('Rust')
--   ON CONFLICT (name) DO NOTHING
--   RETURNING id, name
-- ),
-- new_expertise AS (
--   INSERT INTO expertise (name) VALUES ('Smart Contract Auditing'), ('Zero-Knowledge Proofs')
--   ON CONFLICT (name) DO NOTHING
--   RETURNING id, name
-- )
-- INSERT INTO validator_programming_languages (validator_id, language_id)
-- SELECT v.id, l.id FROM new_validator v, (SELECT id FROM programming_languages WHERE name IN ('Cairo', 'Rust')) l;
--
-- INSERT INTO validator_expertise (validator_id, expertise_id)
-- SELECT v.id, e.id FROM new_validator v, (SELECT id FROM expertise WHERE name IN ('Smart Contract Auditing', 'Zero-Knowledge Proofs')) e;
--
-- COMMIT;
--
-- Sample 2: Update Validator Profile Verification Status (by Admin)
-- UPDATE validator_profiles
-- SET verification = 'verified'
-- WHERE wallet_address = '0x03a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2d';
--
-- Sample 3: Retrieve a Validator Profile with their Skills
-- SELECT
--     vp.government_name,
--     vp.email_address,
--     vp.years_of_experience,
--     vp.verification,
--     ARRAY_AGG(DISTINCT pl.name) AS programming_languages,
--     ARRAY_AGG(DISTINCT ex.name) AS expertise
-- FROM
--     validator_profiles vp
-- LEFT JOIN
--     validator_programming_languages vpl ON vp.id = vpl.validator_id
-- LEFT JOIN
--     programming_languages pl ON vpl.language_id = pl.id
-- LEFT JOIN
--     validator_expertise vex ON vp.id = vex.validator_id
-- LEFT JOIN
--     expertise ex ON vex.expertise_id = ex.id
-- WHERE
--     vp.wallet_address = '0x03a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2d'
-- GROUP BY
--     vp.id;
--


-- Researchers Section
--
-- Sample 1: Researcher Submits a New Report
-- INSERT INTO research_reports (
--   title, project_id, body, researcher_wallet_address
-- )
-- SELECT
--   'Critical Re-entrancy Vulnerability in Staking Contract',
--   p.id,
--   'A detailed description of the re-entrancy vulnerability found in the staking module...',
--   '0x04a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2e'
-- FROM projects p
-- WHERE p.contract_address = '0x02a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2c';
--
-- Sample 2: System Assigns Report to a Validator
-- UPDATE research_reports
-- SET
--   status = 'assigned',
--   validator_wallet_address = '0x03a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2d' -- Validator's wallet
-- WHERE id = (SELECT id FROM research_reports WHERE researcher_wallet_address = '0x04a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2e' LIMIT 1);
--
-- Sample 3: Validator Accepts the Report and Assigns Severity
-- UPDATE research_reports
-- SET
--   status = 'accepted',
--   severity = 'critical',
--   allocated_reward = 2500.00,
--   validator_notes = 'Vulnerability confirmed. Excellent finding and detailed report.'
-- WHERE id = (SELECT id FROM research_reports WHERE researcher_wallet_address = '0x04a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2e' LIMIT 1); -- Use specific report ID
--
-- Sample 4: Retrieve All Reports for a Specific Project
-- SELECT
--     rr.id,
--     rr.title,
--     rr.status,
--     rr.severity,
--     p.name AS project_name,
--     rr.researcher_wallet_address,
--     rr.validator_wallet_address
-- FROM
--     research_reports rr
-- JOIN
--     projects p ON rr.project_id = p.id
-- WHERE
--     p.contract_address = '0x02a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2c';
--


-- Support Ticket Section
--
-- Sample 1: A user submits a new request ticket.
-- The ticket starts with an 'open' status and is not assigned to any agent yet.
-- INSERT INTO request_tickets (
--   subject,
--   message,
--   opened_by
-- ) VALUES (
--   'Problem with Escrow Withdrawal',
--   'I tried to withdraw 500 STRK from my escrow balance, but the transaction failed. My balance was not updated. Can you please investigate?',
--   '0x05a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2f' -- User's wallet address
-- );
--
-- Sample 2: A support agent assigns the ticket to themselves.
-- The status is updated to 'assigned' and the agent's wallet address is recorded.
-- In a real application, you would use the specific ticket ID from the INSERT above.
-- UPDATE request_tickets
-- SET
--   status = 'assigned',
--   assigned_to_agent_address = '0x06a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a3a' -- Agent's wallet address
-- WHERE
--   subject = 'Problem with Escrow Withdrawal' AND status = 'open';
--
-- Sample 3: The agent needs more information and updates the status.
-- UPDATE request_tickets
-- SET
--   status = 'awaiting_user',
--   resolution_response = 'Could you please provide the transaction hash for the failed withdrawal attempt?'
-- WHERE
--   id = '...'; -- Use the specific ticket UUID here
--
-- Sample 4: The support agent resolves the ticket after investigation.
-- The status is set to 'resolved', a final response is provided, and the resolved_at timestamp is set.
-- UPDATE request_tickets
-- SET
--   status = 'resolved',
--   resolution_response = 'We have identified a temporary issue with the payment gateway. The withdrawal has been manually processed and the funds should appear in your wallet shortly. We apologize for the inconvenience.',
--   resolved_at = NOW()
-- WHERE
--   id = '...'; -- Use the specific ticket UUID here
--
-- Sample 5: The user reopens the ticket because the issue persists.
-- The status is updated to 'reopened'. The resolution response and timestamp could be cleared if desired.
-- UPDATE request_tickets
-- SET
--   status = 'reopened',
--   resolved_at = NULL
-- WHERE
--   id = '...'; -- Use the specific ticket UUID here
--
-- Sample 6: Retrieve all active tickets for a support agent's dashboard.
-- This fetches all tickets that are not yet resolved or closed, ordered by creation date.
-- SELECT
--   id,
--   subject,
--   status,
--   opened_by_address,
--   created_at
-- FROM
--   request_tickets
-- WHERE
--   status IN ('open', 'assigned', 'in_progress', 'awaiting_user', 'reopened')
-- ORDER BY
--   created_at ASC;
--
-- Sample 7: Retrieve a specific user's ticket history.
-- SELECT
--   id,
--   subject,
--   status,
--   created_at,
--   resolved_at
-- FROM
--   request_tickets
-- WHERE
--   opened_by_address = '0x05a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2f'
-- ORDER BY
--   created_at DESC;
