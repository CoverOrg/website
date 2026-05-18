CREATE TYPE channels AS ENUM(
    'in_app',
    'whatsapp',
    'sms'
);

CREATE TYPE types AS ENUM (
    'deal_update',
    'scam_alert'
);

CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) NOT NULL,
    deal_id UUID REFERENCES deals(id),
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    notification_type types NOT NULL,
    channel channels NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    sent_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT now()
);
