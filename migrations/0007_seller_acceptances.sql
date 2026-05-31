CREATE TYPE pay_out_methods AS ENUM (
    'easypaisa',
    'jazzcash',
    'nayapay',
    'sadapay',
    'raast',
);

CREATE TYPE courier_services AS ENUM (
    'tcs',
    'leopards'
);

CREATE TYPE decisions AS ENUM (
    'accepted',
    'rejected'
);

CREATE TABLE seller_acceptances (
  id                UUID              PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id          UUID              NOT NULL UNIQUE REFERENCES orders(id) ON DELETE CASCADE,
  seller_name       VARCHAR(120)      NOT NULL,
  payout_method     pay_out_methods   NOT NULL,
  payout_account    VARCHAR(30),      -- mobile number or IBAN
  payout_holder     VARCHAR(120),
  bank_name         VARCHAR(80),
  tracking_id       VARCHAR(80)       NOT NULL,
  courier           courier_services  NOT NULL,
  user_id           UUID              REFERENCES users(id),
  decision          decisions         NOT NULL DEFAULT 'accepted',
  rejection_reason  TEXT,
  accepted_at       TIMESTAMPTZ       NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sa_order   ON seller_acceptances(order_id);
CREATE INDEX idx_sa_user    ON seller_acceptances(user_id);
