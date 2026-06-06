CREATE TYPE doc_types AS ENUM (
    'id_front',
    'id_back',
    'selfie_video'
);

CREATE TYPE kyc_status AS ENUM (
    'pending',
    'approved',
    'rejected'
);

CREATE TABLE IF NOT EXISTS kyc_documents (
  id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id        UUID         NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  doc_type       doc_types    NOT NULL,
  file_url       TEXT         NOT NULL,
  status         kyc_status   NOT NULL DEFAULT 'pending',
  reviewer_note  TEXT,
  submitted_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
  reviewed_at    TIMESTAMPTZ
);

CREATE INDEX idx_kyc_user   ON kyc_documents(user_id);
CREATE INDEX idx_kyc_status ON kyc_documents(status);
