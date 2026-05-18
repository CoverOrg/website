CREATE TYPE deal_types AS ENUM (
    'delivery',
    'milestone',
    'time_window',
    'approval'
);

CREATE TYPE payment_methods AS ENUM (
    'easypaisa',
    'nayapay',
    'raast',
    'bank'
);

CREATE TYPE deal_statuses AS ENUM (
    'draft',
    'pending_payment',
    'payment_received',
    'seller_notified',
    'in_transit',
    'delivered',
    'confirmed',
    'disputed',
    'refunded',
    'cancelled'
);

CREATE TYPE courier_services AS ENUM (
    'tcs',
    'leopard'
);

CREATE TABLE IF NOT EXISTS deals (
    id UUID PRIMARY KEY,
    ref_code TEXT UNIQUE NOT NULL,
    buyer_id UUID REFERENCES users(id) NOT NULL,
    seller_id UUID REFERENCES sellers(id),
    seller_phone TEXT NOT NULL,
    item_name TEXT NOT NULL,
    item_pics BYTEA NOT NULL,
    delivery_video BYTEA NOT NULL,
    amount BIGINT NOT NULL,
    fee_amount BIGINT NOT NULL DEFAULT 0,
    fee_percent INTEGER NOT NULL DEFAULT 0,
    deal_type deal_types NOT NULL,
    payment_method payment_methods NOT NULL,
    deal_status deal_statuses NOT NULL,
    risk_score INTEGER NOT NULL DEFAULT 0,
    courier courier_services NOT NULL,
    tracking_number TEXT,
    tracking_verified BOOLEAN DEFAULT FALSE NOT NULL,
    expected_delivery TIMESTAMPTZ,
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    confirmed_at TIMESTAMPTZ,
    disputed_at TIMESTAMPTZ,
    refunded_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ
);
