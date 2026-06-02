CREATE TABLE IF NOT EXISTS delivery_confirmations (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id        UUID NOT NULL UNIQUE REFERENCES orders(id) ON DELETE CASCADE,
  video_url       TEXT,
  notes           TEXT,
  qr_scanned_at   TIMESTAMPTZ,
  confirmed_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
