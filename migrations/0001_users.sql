CREATE TYPE role_types AS ENUM (
    'buyer',
    'seller',
    'admin'
);

CREATE TYPE user_city AS ENUM (
    'islamabad',
    'lahore',
    'karachi',
    'faisalabad',
    'quetta',
    'peshawar'
);

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    phone TEXT UNIQUE NOT NULL,
    city user_city NOT NULL,
    role role_types NOT NULL,
    trust_score INTEGER DEFAULT 0,
    deal_count INTEGER DEFAULT 0,
    dispute_count INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT now(),
    last_active TIMESTAMPTZ
);
