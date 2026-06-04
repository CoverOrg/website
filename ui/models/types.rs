use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "user_city", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UserCity {
    Islamabad,
    Lahore,
    Karachi,
    Faisalabad,
    Quetta,
    Peshawar,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "user_kyc_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UserKycStatus {
    NotSubmitted,
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "otp_purpose", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OtpPurpose {
    Signup,
    Login,
    ChangePhone,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "method_types", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MethodTypes {
    Easypaisa,
    Jazzcash,
    Nayapay,
    Sadapay,
    Raast,
    Bank,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "bank_names", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BankNames {
    Hbl,
    Ubl,
    Mcb,
    AlliedBank,
    BankAlfalah,
    MeezanBank,
    AskariBank,
    BankAlHabib,
    FaysalBank,
    SoneriBank,
    JsBank,
    Silkbank,
    SummitBank,
    Bankislami,
    DubaiIslamicBank,
    StandardChartered,
    SambaBank,
    Nbp,
    HabibMetropolitan,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "doc_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DocType {
    IDFront,
    IDBack,
    SelfieVideo,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "kyc_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum KycStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Created,
    Paid,
    SellerConfirmed,
    Shipped,
    Delivered,
    Released,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "seller_decision", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SellerDecision {
    Accepted,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "referral_methods", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ReferralMethods {
    SocialMedia,
    WhatsappGroups,
    YouTube,
    Blog,
    WordOfMouth,
    MarketplaceCommunities,
    Other,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "application_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ApplicationStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "reach_estimated", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ReachEstimated {
    Under100,
    From100To500,
    From500To2000,
    From2000To10000,
    From10000AndPlus,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "earning_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EarningStatus {
    Pending,
    PaidOut,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "pay_out_methods", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PayoutMethods {
    Easypaisa,
    Jazzcash,
    Nayapay,
    Sadapay,
    Raast,
    Bank,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "courier_services", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CourierServices {
    TCS,
    Leopard,
}
