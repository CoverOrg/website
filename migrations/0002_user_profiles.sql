CREATE TABLE IF NOT EXISTS user_profiles (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name             VARCHAR(120),
    city             user_city   NOT NULL,
    avatar_url       TEXT,
    is_buyer         BOOLEAN     NOT NULL DEFAULT TRUE,
    is_seller        BOOLEAN     NOT NULL DEFAULT FALSE,
    seller_handle    VARCHAR(30) NOT NULL UNIQUE,
    id_card          VARCHAR(20) UNIQUE,          -- set when KYC submitted
    kyc_status       user_kyc_status NOT NULL DEFAULT 'not_submitted',
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_users_handle ON user_profiles(seller_handle) WHERE seller_handle IS NOT NULL;
