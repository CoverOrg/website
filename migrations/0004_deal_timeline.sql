CREATE type deal_event_type AS ENUM (
    'deal_created',
    'payment_received',
    'payment_failed',
    'seller_notified',
    'seller_accepted',
    'seller_rejected',
    'shipped',
    'in_transit',
    'out_for_delivery',
    'delivered',
    'buyer_confirmed',
    'dispute_raised',
    'dispute_resolved',
    'refunded',
    'deal_cancelled'
);

CREATE TYPE deal_actor AS ENUM (
    'buyer',
    'seller',
    'admin',
    'system'
);

CREATE TABLE IF NOT EXISTS deal_timeline (
    id UUID PRIMARY KEY,
    deal_id UUID REFERENCES deals(id) NOT NULL,
    event_type deal_event_type NOT NULL,
    description TEXT,
    actor deal_actor NOT NULL,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL
);
