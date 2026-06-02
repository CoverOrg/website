CREATE TYPE pay_out_methods AS ENUM (
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

CREATE TYPE courier_services AS ENUM (
    'tcs',
    'leopards'
);

CREATE TYPE decisions AS ENUM (
    'accepted',
    'rejected'
);

CREATE TABLE IF NOT EXISTS seller_acceptances (
  id                UUID             PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id          UUID             NOT NULL UNIQUE REFERENCES orders(id) ON DELETE CASCADE,
  seller_name       VARCHAR(120)     NOT NULL,
  payout_method     pay_out_method   NOT NULL,
  payout_account    VARCHAR(30),
  iban              VARCHAR(30),
  payout_holder     VARCHAR(120),
  bank_name         bank_names,
  tracking_id       VARCHAR(80)      NOT NULL,
  courier           courier_service  NOT NULL,
  user_id           UUID             REFERENCES users(id),
  decision          seller_decision  NOT NULL DEFAULT 'accepted',
  rejection_reason  TEXT,
  accepted_at       TIMESTAMPTZ      NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sa_order   ON seller_acceptances(order_id);
CREATE INDEX idx_sa_user    ON seller_acceptances(user_id);
