CREATE TABLE delivery_confirmations (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id        UUID NOT NULL UNIQUE REFERENCES orders(id) ON DELETE CASCADE,
  video_url       TEXT,
  notes           TEXT,
  confirmed_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
