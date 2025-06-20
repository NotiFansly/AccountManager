//use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// Removed unused tauri imports: Menu, MenuItem, MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent, Emitter, Manager, State, WindowEvent
use reqwest; // Add reqwest for AppState
             //use tokio::sync::Mutex; // Add Mutex for AppState

// AppState definition
pub struct AppState {
    pub client: reqwest::Client,
    pub api_base_url: String,
}

// All structs and their fields are now public
/*#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub fansly_user_id: String,
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub is_creator: bool,
}*/

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    #[serde(rename = "fanslyProfile")]
    pub fansly_profile: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountResponse {
    pub account_id: String,
    pub password: String,
    pub sync_key: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncDataRequest {
    pub sync_key: String,
    pub data_type: String,
    pub data: serde_json::Value,
    pub sync_mode: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone as it was for SubscriptionTier
pub struct FanslyAccountData {
    pub id: String,
    pub email: String,
    pub username: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "followCount")]
    pub follow_count: i32,
    #[serde(rename = "subscriberCount")]
    pub subscriber_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionTier {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub name: String,
    pub color: String,
    pub pos: i32,
    pub price: i32,
    #[serde(rename = "maxSubscribers")]
    pub max_subscribers: i32,
    #[serde(rename = "subscriptionBenefits")]
    pub subscription_benefits: Vec<String>,
    #[serde(rename = "includedTierIds")]
    pub included_tier_ids: Vec<String>,
    pub plans: Vec<SubscriptionPlan>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionPlan {
    pub id: String,
    pub status: i32,
    #[serde(rename = "billingCycle")]
    pub billing_cycle: i32,
    pub price: i32,
    #[serde(rename = "useAmounts")]
    pub use_amounts: i32,
    pub promos: Vec<serde_json::Value>, // Keep as generic for now
    pub uses: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone for consistency
pub struct FanslyAccountResponse {
    pub success: bool,
    pub response: Vec<FanslyAccountDetails>,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone for consistency
pub struct FanslyAccountDetails {
    pub id: String,
    pub username: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "subscriptionTiers")]
    pub subscription_tiers: Vec<SubscriptionTier>,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone for consistency
pub struct FanslyResponse {
    pub success: bool,
    pub response: FanslyResponseData,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone for consistency
pub struct FanslyResponseData {
    pub account: FanslyAccountData,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone for consistency
pub struct FanslyFollowerResponse {
    pub success: bool,
    pub response: Vec<FanslyFollower>,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone for consistency
pub struct FanslyFollower {
    #[serde(rename = "followerId")]
    pub follower_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone for consistency
pub struct FanslySubscriberResponse {
    pub success: bool,
    pub response: FanslySubscriberData,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone for consistency
pub struct FanslySubscriberData {
    pub subscriptions: Vec<FanslySubscriber>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FanslySubscriber {
    pub id: String,
    #[serde(rename = "historyId")]
    pub history_id: String,
    #[serde(rename = "subscriberId")]
    pub subscriber_id: String,
    #[serde(rename = "subscriptionTierId")]
    pub subscription_tier_id: String,
    #[serde(rename = "subscriptionTierName")]
    pub subscription_tier_name: String,
    #[serde(rename = "subscriptionTierColor")]
    pub subscription_tier_color: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "promoId")]
    pub promo_id: Option<String>,
    #[serde(rename = "giftCodeId")]
    pub gift_code_id: Option<String>,
    #[serde(rename = "paymentMethodId")]
    pub payment_method_id: String,
    pub status: i32,
    pub price: i32,
    #[serde(rename = "renewPrice")]
    pub renew_price: i32,
    #[serde(rename = "renewCorrelationId")]
    pub renew_correlation_id: String,
    #[serde(rename = "autoRenew")]
    pub auto_renew: i32,
    #[serde(rename = "billingCycle")]
    pub billing_cycle: i32,
    pub duration: i32,
    #[serde(rename = "renewDate")]
    pub renew_date: i64,
    pub version: i32,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    #[serde(rename = "endsAt")]
    pub ends_at: i64,
    #[serde(rename = "promoPrice")]
    pub promo_price: Option<i32>,
    #[serde(rename = "promoDuration")]
    pub promo_duration: Option<i32>,
    #[serde(rename = "promoStatus")]
    pub promo_status: Option<i32>,
    #[serde(rename = "promoStartsAt")]
    pub promo_starts_at: Option<i64>,
    #[serde(rename = "promoEndsAt")]
    pub promo_ends_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone for consistency
pub struct SyncResponse {
    pub success: bool,
    pub message: String,
    pub data_type: String,
    pub count: i32,
    pub timestamp: String,
}
