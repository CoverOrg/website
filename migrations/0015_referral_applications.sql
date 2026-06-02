CREATE TYPE referral_methods AS ENUM (
    'social_media',
    'whatsapp_groups',
    'youtube',
    'blog',
    'word_of_mouth',
    'marketplace_communities',
    'other'
);

CREATE TYPE reach_estimated AS ENUM (
    'under_100',
    '100_500',
    '500_2000',
    '2000_10000',
    '10000_plus'
);

CREATE TYPE application_status AS ENUM (
    'pending',
    'approved',
    'rejected'
)

CREATE TYPE method_types AS ENUM (
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

CREATE TABLE referral_applications (
  id               UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id          UUID        NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
  full_name        VARCHAR(120) NOT NULL,
  whatsapp         VARCHAR(15)  NOT NULL,
  referral_method  referral_methods  NOT NULL,
  estimated_reach  reach_estimated  NOT NULL,

  -- Payout for referral rewards (separate from buyer/seller payout)
  payout_method    method_types  NOT NULL,
  payout_account   VARCHAR(30),
  iban             VARCHAR(30),
  payout_holder    VARCHAR(120),
  bank_name        VARCHAR(80),

  -- Unique referral link: cover.mom/r/{referral_code}
  referral_code    VARCHAR(12)  UNIQUE,
  -- Generated on approval, NULL until then

  status           application_status  NOT NULL DEFAULT 'pending',
  reviewed_at      TIMESTAMPTZ,
  rejection_reason TEXT,
  created_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
  updated_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ref_app_user ON referral_applications(user_id);
CREATE INDEX idx_ref_app_code ON referral_applications(referral_code)
  WHERE referral_code IS NOT NULL;
