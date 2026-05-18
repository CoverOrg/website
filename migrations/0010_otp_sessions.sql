CREATE TABLE IF NOT EXISTS otp_sessions (
    id UUID PRIMARY KEY,
    phone TEXT NOT NULL,
    code INTEGER NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT now()
);
