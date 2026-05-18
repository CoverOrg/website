CREATE TYPE severity AS ENUM (
    'low',
    'medium',
    'high',
    'critical'
);

CREATE TYPE scam_city AS ENUM (
    'islamabad',
    'lahore',
    'karachi',
    'faisalabad',
    'quetta',
    'peshawar'
);

CREATE TYPE scam_platform AS ENUM (
    'facebook',
    'whats_spp',
    'instagram',
    'olx',
    'daraz',
    'other'
);

CREATE TABLE IF NOT EXISTS scam_alerts (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    city scam_city NOT NULL,
    platform scam_platform NOT NULL,
    description TEXT NOT NULL,
    scammer_phone TEXT,
    amount_lost BIGINT,
    victim_count INTEGER DEFAULT 1,
    severity severity NOT NULL,
    is_published BOOLEAN DEFAULT FALSE,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT now()
);
