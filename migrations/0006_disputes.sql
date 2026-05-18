CREATE TYPE issue_types AS ENUM (
    'not_delivered',
    'wrong_item',
    'damaged',
    'fake_counterfeit',
    'not_as_described',
    'other'
);

CREATE TYPE dispute_statuses AS ENUM (
    'open',
    'awaiting_seller',
    'under_review',
    'resolved',
    'appealed'
);

CREATE TYPE verdicts as ENUM (
    'buyer_wins',
    'seller_wins',
    'partial',
    'return_refund'
);

CREATE TABLE IF NOT EXISTS disputes (
    id UUID PRIMARY KEY,
    deal_id UUID REFERENCES deals(id) NOT NULL,
    raised_by UUID REFERENCES users(id) NOT NULL,
    admin_id UUID REFERENCES users(id),
    issue_type issue_types NOT NULL,
    description TEXT NOT NULL,
    buyer_evidence JSONB,
    seller_evidence JSONB,
    status dispute_statuses NOT NULL DEFAULT 'open',
    verdict verdicts,
    verdict_note TEXT,
    verdict_amount BIGINT,
    created_at TIMESTAMPTZ NOT NULL default now(),
    resolved_at TIMESTAMPTZ
);
