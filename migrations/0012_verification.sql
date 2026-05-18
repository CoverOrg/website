CREATE TABLE IF NOT EXISTS verifications (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id) NOT NULL,
    image TEXT,
    id_card_front TEXT,
    id_card_back TEXT,
    is_verified BOOLEAN DEFAULT FALSE
);
