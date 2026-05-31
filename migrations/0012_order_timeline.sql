CREATE TYPE order_timeline_status AS ENUM (
    'created',
    'paid',
    'seller_confirmed',
    'shipped',
    'delivered',
    'released',
    'disputed',
    'cancelled',
    'refunded'
);

CREATE TABLE order_timeline (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id    UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  status      order_timeline_status NOT NULL,
  note        TEXT,
  actor_id    UUID REFERENCES users(id),   -- who triggered it (NULL = system)
  actor_hint  VARCHAR(120),
  created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_timeline_order ON order_timeline(order_id, created_at);
