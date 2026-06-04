CREATE TABLE IF NOT EXISTS buyer_payout_methods (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  method_type     method_types NOT NULL,
  account_number  INTEGER,
  iban            VARCHAR(34),
  account_holder  VARCHAR(120),
  bank_name       bank_names,
  is_default      BOOLEAN NOT NULL DEFAULT FALSE,
  created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_buyer_pm_user ON buyer_payout_methods(user_id);
-- Ensure only one default per user
CREATE UNIQUE INDEX idx_payout_pm_default ON buyer_payout_methods(user_id)
    WHERE is_default = TRUE;
