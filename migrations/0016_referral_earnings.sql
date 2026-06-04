CREATE TABLE referral_earnings (
  id              UUID            PRIMARY KEY DEFAULT gen_random_uuid(),
  referrer_id     UUID            NOT NULL REFERENCES users(id),
  order_id        UUID            NOT NULL UNIQUE REFERENCES orders(id),
  cover_fee       BIGINT          NOT NULL,    -- Cover's fee on this order
  earning_amount  BIGINT          NOT NULL,    -- 20% of cover_fee
  status          earning_status  NOT NULL DEFAULT 'pending',
  paid_out_at     TIMESTAMPTZ,
  created_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ref_earn_referrer ON referral_earnings(referrer_id);
CREATE INDEX idx_ref_earn_order    ON referral_earnings(order_id);
