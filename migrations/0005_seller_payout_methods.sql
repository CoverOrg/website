CREATE TYPE method_types AS ENUM (
    'easypaisa',
    'jazzcash',
    'nayapay',
    'sadapay',
    'raast',
    'bank'
);

CREATE TYPE bank_names AS ENUM (
    'hbl',
    'ubl',
    'mcb',
    'allied_bank',
    'bank_alfalah',
    'meezan_bank',
    'askari_bank',
    'bank_al_habib',
    'faysal_bank',
    'soneri_bank',
    'js_bank',
    'silkbank',
    'summit_bank',
    'bankislami',
    'dubai_islamic_bank',
    'standard_chartered',
    'samba_bank',
    'nbp',
    'habib_metropolitan'
);

CREATE TABLE IF NOT EXISTS seller_payout_methods (
  id             UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id        UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  method_type    method_types NOT NULL,
  account_number VARCHAR(30),
  iban           VARCHAR(34),
  account_holder VARCHAR(120) NOT NULL,
  bank_name      VARCHAR(80),
  is_default     BOOLEAN     NOT NULL DEFAULT FALSE,
  created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_seller_pm_user ON seller_payout_methods(user_id);
CREATE UNIQUE INDEX idx_seller_pm_default ON seller_payout_methods(user_id)
    WHERE is_default = TRUE;
