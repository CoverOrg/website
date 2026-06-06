CREATE TABLE IF NOT EXISTS users (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    phone            VARCHAR(15) NOT NULL UNIQUE,
    name             VARCHAR(120),
    city             user_city,
    avatar_url       TEXT,
    is_buyer         BOOLEAN     NOT NULL DEFAULT TRUE,
    is_seller        BOOLEAN     NOT NULL DEFAULT FALSE,
    kyc_status       user_kyc_status NOT NULL DEFAULT 'not_submitted',
    seller_handle    VARCHAR(30) UNIQUE,
    id_card          VARCHAR(20) UNIQUE,          -- set when KYC submitted
    phone_verified   BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_users_phone ON users(phone);
CREATE UNIQUE INDEX idx_users_handle ON users(seller_handle) WHERE seller_handle IS NOT NULL;
