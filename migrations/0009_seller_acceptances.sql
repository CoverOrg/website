CREATE TABLE IF NOT EXISTS seller_acceptances (
  id                UUID             PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id          UUID             NOT NULL UNIQUE REFERENCES orders(id) ON DELETE CASCADE,
  seller_name       VARCHAR(120)     NOT NULL,
  payout_method     pay_out_methods  NOT NULL,
  payout_account    VARCHAR(30),
  iban              VARCHAR(30),
  payout_holder     VARCHAR(120),
  bank_name         bank_names,
  tracking_id       VARCHAR(80)      NOT NULL,
  courier           courier_services NOT NULL,
  user_id           UUID             REFERENCES users(id),
  decision          seller_decision  NOT NULL DEFAULT 'accepted',
  rejection_reason  TEXT,
  accepted_at       TIMESTAMPTZ      NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sa_order   ON seller_acceptances(order_id);
CREATE INDEX idx_sa_user    ON seller_acceptances(user_id);
