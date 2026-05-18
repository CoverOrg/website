CREATE TYPE risk_sources AS ENUM (
    'facebook',
    'reddit',
    'twitter',
    'linkedin'
);

CREATE TYPE risk_severities AS ENUM (
    'low',
    'medium',
    'high'
);

CREATE TABLE IF NOT EXISTS risk_reports (
    id UUID PRIMARY KEY,
    seller_id UUID REFERENCES sellers(id) NULL,
    phone TEXT NOT NULL,
    risk_source risk_sources NOT NULL,
    description TEXT NOT NULL,
    risk_severity risk_severities NOT NULL,
    evidence_url TEXT,
    verified BOOLEAN DEFAULT FALSE,
    verified_at TIMESTAMPTZ,
    reported_at TIMESTAMPTZ DEFAULT now()
);
