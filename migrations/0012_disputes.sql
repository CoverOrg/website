CREATE TABLE IF NOT EXISTS disputes (
  id               UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id         UUID           NOT NULL UNIQUE REFERENCES orders(id) ON DELETE CASCADE,
  raised_by        UUID           NOT NULL REFERENCES users(id),
  reason           dispute_reason NOT NULL,
  description      TEXT           NOT NULL,
  proof_urls       TEXT[]         NOT NULL DEFAULT '{}',  -- S3 keys, up to 5
  status           dispute_status NOT NULL DEFAULT 'open',
  resolution_notes TEXT,
  admin_id         UUID           REFERENCES users(id),   -- who resolved it
  created_at       TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
  updated_at       TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
  resolved_at      TIMESTAMPTZ
);

CREATE INDEX idx_disputes_order  ON disputes(order_id);
CREATE INDEX idx_disputes_status ON disputes(status);
