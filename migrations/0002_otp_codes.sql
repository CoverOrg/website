CREATE TYPE otp_purpose AS ENUM (
    'signup',
    'login',
    'change_phone'
);

CREATE TABLE otp_codes (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  phone       VARCHAR(15) NOT NULL,
  code        VARCHAR(6)  NOT NULL,
  purpose     otp_purpose NOT NULL,
  expires_at  TIMESTAMPTZ NOT NULL,
  used_at     TIMESTAMPTZ,
  attempts    SMALLINT    NOT NULL DEFAULT 0, -- increment on wrong guess; reject after 5
  created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_otp_phone_purpose ON otp_codes(phone, purpose, expires_at);
