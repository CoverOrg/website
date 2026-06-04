CREATE TYPE user_city AS ENUM (
    'islamabad',
    'lahore',
    'karachi',
    'faisalabad',
    'quetta',
    'peshawar'
);

CREATE TYPE user_kyc_status AS ENUM (
    'not_submitted',
    'pending',
    'approved',
    'rejected'
);

CREATE TYPE otp_purpose AS ENUM (
    'signup',
    'login',
    'change_phone'
);

CREATE TYPE method_types AS ENUM (
    'easypaisa',
    'jazzcash',
    'nayapay',
    'sadapay',
    'raast',
    'bank'
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

CREATE TYPE courier_services AS ENUM (
    'tcs',
    'leopards'
);

CREATE TYPE seller_decision AS ENUM (
    'accepted',
    'rejected'
);

CREATE TYPE order_status AS ENUM (
    'created',
    'paid',
    'seller_confirmed',
    'shipped',
    'delivered',
    'released'
);

CREATE TYPE dispute_reason AS ENUM (
    'item_not_received',
    'not_as_described',
    'damaged',
    'seller_not_responding',
    'suspected_scam',
    'other'
);

CREATE TYPE dispute_status AS ENUM (
    'open',
    'under_review',
    'resolved_refund',
    'resolved_release',
    'closed'
);

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

CREATE TYPE actor_type AS ENUM (
    'buyer',
    'seller',
    'admin',
    'system'
);

CREATE TYPE notification_types AS ENUM (
    'order_paid',
    'order_confirmed',
    'order_shipped',
    'order_rejected',
    'delivery_confirmed',
    'payment_released',
    'dispute_opened',
    'dispute_resolved',
    'kyc_approved',
    'kyc_rejected',
    'referrel_approved',
    'referrel_earned'
);

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
);

CREATE TYPE earning_status AS ENUM (
    'pending',
    'paid_out',
    'cancelled'
);
