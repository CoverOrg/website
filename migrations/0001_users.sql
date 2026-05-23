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
    full_name TEXT NOT NULL,
    username TEXT UNIQUE NOT NULL,
    phone TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    city user_city NOT NULL,
    bio TEXT NOT NULL,
    avatar_url TEXT,
    trust_score INTEGER DEFAULT 0,
    kyc_level INTEGER DEFAULT 0,
    is_seller BOOL NOT NULL,
    is_verified BOOL DEFAULT FALSE,
    deals_completed INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT now()
);
