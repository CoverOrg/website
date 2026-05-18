CREATE TYPE select_category AS ENUM (
    'clothing',
    'jewellery',
    'shoes'
);

CREATE TABLE IF NOT EXISTS sellers (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    shop_name TEXT NOT NULL,
    banner_url TEXT NOT NULL,
    category select_category NOT NULL,
    address TEXT NOT NULL,
    description TEXT NOT NULL,
    risk_score INTEGER DEFAULT 0,
    risk_checked_at TIMESTAMPTZ,
    is_featured BOOLEAN DEFAULT FALSE,
    deal_count INTEGER DEFAULT 0,
    became_seller TIMESTAMPTZ DEFAULT now()
);
