use reqwest;
use serde::{Deserialize, Serialize};

// The application state, holds a shared reqwest client and the API base URL.
pub struct AppState {
    pub client: reqwest::Client,
    pub api_base_url: String,
}

// --- Fansly API Models ---
// These structs are for parsing responses directly from Fansly's API.

// --- FIX: Added Serialize ---
// This is returned by the `get_fansly_profile` command.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FanslyProfile {
    pub id: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct FanslyProfileWrapper {
    pub account: FanslyProfile,
}

#[derive(Deserialize, Debug)]
pub struct FanslyProfileResponse {
    pub success: bool,
    pub response: FanslyProfileWrapper,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SubscriptionTier {
    pub id: String,
    pub name: String,
    pub price: i32, // in cents
}

#[derive(Deserialize, Debug)]
pub struct AccountDetails {
    #[serde(rename = "subscriptionTiers")]
    pub subscription_tiers: Vec<SubscriptionTier>,
}

#[derive(Deserialize, Debug)]
pub struct AccountDetailsResponse {
    pub success: bool,
    pub response: Vec<AccountDetails>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct FanslyFollower {
    #[serde(rename = "followerId")]
    pub follower_id: String,
}

#[derive(Deserialize, Debug)]
pub struct FanslyFollowerResponse {
    pub success: bool,
    pub response: Vec<FanslyFollower>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct FanslySubscriber {
    #[serde(rename = "subscriberId")]
    pub subscriber_id: String,
    #[serde(rename = "subscriptionTierId")]
    pub subscription_tier_id: String,
    #[serde(rename = "endsAt")]
    pub ends_at: i64,
    pub status: i32,
}

#[derive(Deserialize, Debug)]
pub struct SubscriberData {
    pub subscriptions: Vec<FanslySubscriber>,
}

#[derive(Deserialize, Debug)]
pub struct FanslySubscriberResponse {
    pub success: bool,
    pub response: SubscriberData,
}

// --- Backend API Models ---
// These structs are for communicating with YOUR new API service.

#[derive(Serialize)]
pub struct VerifyCreatorRequest<'a> {
    #[serde(rename = "fanslyProfile")]
    pub fansly_profile: &'a FanslyProfile,
}

// --- FIX: Added Serialize ---
// This is returned by the `verify_creator_account` command.
#[derive(Deserialize, Debug, Serialize)]
pub struct VerifyCreatorResponse {
    pub sync_key: String,
}

#[derive(Serialize)]
pub struct SyncDataRequest<T> {
    pub sync_key: String,
    pub data_type: String,
    pub data: T,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct SyncDataResponse {
    pub success: bool,
    pub message: String,
    pub data_type: String,
    pub count: i32,
    pub timestamp: String,
}
