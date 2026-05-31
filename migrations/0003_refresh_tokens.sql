CREATE TABLE refresh_tokens (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id     UUID          NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  token_hash  CHAR(64)      NOT NULL UNIQUE,  -- SHA-256 of the raw token
  device_hint VARCHAR(200),                   -- UA snippet, optional
  expires_at  TIMESTAMPTZ   NOT NULL,
  revoked_at  TIMESTAMPTZ,
  created_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_rt_user ON refresh_tokens(user_id);
CREATE INDEX idx_rt_hash ON refresh_tokens(token_hash);
