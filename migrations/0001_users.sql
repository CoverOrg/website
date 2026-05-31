CREATE TYPE user_city AS ENUM (
    'islamabad',
    'lahore',
    'karachi',
    'faisalabad',
    'quetta',
    'peshawar'
);

CREATE TYPE user_kyc_status AS ENUM (
    'none',
    'pending',
    'approved',
    'rejected'
);

CREATE TABLE IF NOT EXISTS users (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    phone            VARCHAR(15) NOT NULL UNIQUE,
    name             VARCHAR(120),
    city             user_city   NOT NULL,
    avatar_url       TEXT,
    is_buyer         BOOLEAN     NOT NULL DEFAULT TRUE,
    is_seller        BOOLEAN     NOT NULL DEFAULT FALSE,
    kyc_status       user_kyc_status NOT NULL DEFAULT 'none',
    id_card          VARCHAR(20) UNIQUE,          -- set when KYC submitted
    phone_verified   BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_phone ON users(phone);
