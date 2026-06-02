CREATE TYPE courier_services AS ENUM (
    'tcs',
    'leopards'
);

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

CREATE TABLE shipments (
  id                    UUID             PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id              UUID             NOT NULL UNIQUE REFERENCES orders(id) ON DELETE CASCADE,
  seller_acceptance_id  UUID             REFERENCES seller_acceptances(id),
  tracking_id           VARCHAR(80)      NOT NULL,
  courier               courier_services NOT NULL,
  handover_video_url    TEXT,
  payout_method         pay_out_methods  NOT NULL,
  payout_account        VARCHAR(30),
  payout_holder         VARCHAR(120),
  bank_name             bank_names,
  delivery_qr_token     VARCHAR(32)      NOT NULL,
  shipped_at            TIMESTAMPTZ      NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_shipments_order ON shipments(order_id);
