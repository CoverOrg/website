CREATE TYPE gateways AS ENUM (
    'easypaisa',
    'nayapay',
    'raast',
    'bank'
);

CREATE TYPE payment_status AS ENUM (
    'pending',
    'received',
    'held',
    'released',
    'refunded'
);

CREATE TABLE IF NOT EXISTS payments (
    id UUID PRIMARY KEY,
    deal_id UUID REFERENCES deals(id) NOT NULL,
    gateway gateways NOT NULL,
    amount BIGINT NOT NULL,
    fee BIGINT NOT NULL DEFAULT 0,
    status payment_status NOT NULL,
    gateway_ref TEXT,
    screenshot_url TEXT,
    paid_at TIMESTAMPTZ,
    released_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL
);
