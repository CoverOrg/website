CREATE TABLE IF NOT EXISTS orders (
  id                       UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
  order_number             VARCHAR(14)   UNIQUE NOT NULL,  -- 'COV-XXXX-XXXX'
  buyer_id                 UUID          NOT NULL REFERENCES users(id),
  seller_id                UUID          REFERENCES users(id),
  status                   VARCHAR(20)   NOT NULL DEFAULT 'created',
  -- State machine:
  -- created → paid → seller_confirmed → shipped → delivered → released
  --                        ↓               ↓            ↓
  --                    rejected        disputed      disputed
  -- created → cancelled (buyer cancels before paying)
  -- disputed → refunded (admin resolution)

  product_name             VARCHAR(255)  NOT NULL,
  product_link             TEXT,
  product_image_url        TEXT,
  product_amount           BIGINT        NOT NULL CHECK (product_amount > 0),
  delivery_charges         BIGINT        NOT NULL DEFAULT 0 CHECK (delivery_charges >= 0),
  cover_fee                BIGINT        NOT NULL, -- = ceil((product_amount + delivery_charges) * 0.05 * 100) / 100
  total_amount             BIGINT        NOT NULL, -- = product_amount + delivery_charges + cover_fee
  seller_payout            BIGINT        NOT NULL, -- = product_amount + delivery_charges  (no fee deducted from seller)
  currency                 CHAR(3)       NOT NULL DEFAULT 'PKR',
  seller_name              VARCHAR(120)  NOT NULL,
  seller_whatsapp          VARCHAR(20)   NOT NULL,
  seller_handle            VARCHAR(30)   REFERENCES user_profiles(seller_handle),
  -- Token embedded in the WhatsApp link for unauthenticated seller access
  -- cover.mom/{order_number}?t={seller_accept_token}
  seller_accept_token      VARCHAR(32)   NOT NULL DEFAULT '',
  delivery_qr_token        VARCHAR(32),
  last_seller_notified_at  TIMESTAMPTZ,
  delivery_address         TEXT,
  referred_by              UUID          REFERENCES users(id),
  created_at               TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
  updated_at               TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
  paid_at                  TIMESTAMPTZ,
  confirmed_at             TIMESTAMPTZ,
  shipped_at               TIMESTAMPTZ,
  delivered_at             TIMESTAMPTZ,
  released_at              TIMESTAMPTZ
);

CREATE INDEX idx_orders_buyer       ON orders(buyer_id);
CREATE INDEX idx_orders_seller      ON orders(seller_id);
CREATE INDEX idx_orders_status      ON orders(status);
CREATE INDEX idx_orders_number      ON orders(order_number);
CREATE INDEX idx_orders_accept_tok  ON orders(seller_accept_token);
CREATE INDEX idx_orders_created     ON orders(created_at DESC);
