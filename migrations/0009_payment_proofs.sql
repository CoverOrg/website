CREATE TYPE method_types AS ENUM (
    'easypaisa',
    'jazzcash',
    'nayapay',
    'sadapay',
    'raast',
    'bank'
);

CREATE TABLE IF NOT EXISTS payment_proofs (
  id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id          UUID NOT NULL UNIQUE REFERENCES orders(id) ON DELETE CASCADE,
  transaction_id    UUID NOT NULL,
  method_type       method_types NOT NULL,
  screenshot_url    TEXT,
  submitted_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_pp_order ON payment_proofs(order_id);
