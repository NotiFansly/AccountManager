// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest;
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;
use tauri::State;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct CreateAccountRequest {
    fansly_user_id: String,
    email: String,
    username: String,
    display_name: String,
    is_creator: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateAccountResponse {
    account_id: String,
    password: String,
    sync_key: String,
    user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SyncDataRequest {
    sync_key: String,
    data_type: String,
    data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct FanslyAccountData {
    id: String,
    email: String,
    username: String,
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "followCount")]
    follow_count: i32,
    #[serde(rename = "subscriberCount")]
    subscriber_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SubscriptionTier {
    id: String,
    #[serde(rename = "accountId")]
    account_id: String,
    name: String,
    color: String,
    pos: i32,
    price: i32,
    #[serde(rename = "maxSubscribers")]
    max_subscribers: i32,
    #[serde(rename = "subscriptionBenefits")]
    subscription_benefits: Vec<String>,
    #[serde(rename = "includedTierIds")]
    included_tier_ids: Vec<String>,
    plans: Vec<SubscriptionPlan>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SubscriptionPlan {
    id: String,
    status: i32,
    #[serde(rename = "billingCycle")]
    billing_cycle: i32,
    price: i32,
    #[serde(rename = "useAmounts")]
    use_amounts: i32,
    promos: Vec<serde_json::Value>, // Keep as generic for now
    uses: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct FanslyAccountResponse {
    success: bool,
    response: Vec<FanslyAccountDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FanslyAccountDetails {
    id: String,
    username: String,
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "subscriptionTiers")]
    subscription_tiers: Vec<SubscriptionTier>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FanslyResponse {
    success: bool,
    response: FanslyResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
struct FanslyResponseData {
    account: FanslyAccountData,
}

struct AppState {
    client: reqwest::Client,
    api_base_url: String,
}

#[tauri::command]
async fn create_account(
    state: State<'_, Mutex<AppState>>,
    fansly_user_id: String,
    email: String,
    username: String,
    display_name: String,
    is_creator: bool,
) -> Result<CreateAccountResponse, String> {
    let state = state.lock().await;

    let request = CreateAccountRequest {
        fansly_user_id,
        email,
        username,
        display_name,
        is_creator,
    };

    let response = state
        .client
        .post(&format!("{}/api/v1/accounts", state.api_base_url))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        let account_response: CreateAccountResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(account_response)
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("API error: {}", error_text))
    }
}

#[tauri::command]
async fn fetch_fansly_data(
    state: State<'_, Mutex<AppState>>,
    auth_token: String,
) -> Result<FanslyAccountData, String> {
    let state = state.lock().await;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        auth_token.parse().map_err(|_| "Invalid token")?,
    );
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
            .parse()
            .unwrap(),
    );
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let response = state
        .client
        .get("https://apiv3.fansly.com/api/v1/account/me")
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        let fansly_response: FanslyResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        // Add debugging
        println!(
            "Parsed account data: {:?}",
            fansly_response.response.account
        );
        println!(
            "Display name: {}",
            fansly_response.response.account.display_name
        );

        Ok(fansly_response.response.account)
    } else {
        Err(format!("Fansly API error: {}", response.status()))
    }
}

#[tauri::command]
async fn sync_data(
    state: State<'_, Mutex<AppState>>,
    sync_key: String,
    data_type: String,
    data: serde_json::Value,
) -> Result<String, String> {
    let state = state.lock().await;

    let request = SyncDataRequest {
        sync_key,
        data_type,
        data,
    };

    let response = state
        .client
        .post(&format!("{}/api/v1/sync", state.api_base_url))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        Ok("Data synced successfully".to_string())
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Sync failed: {}", error_text))
    }
}

#[tauri::command]
async fn fetch_followers_and_subscribers(
    state: State<'_, Mutex<AppState>>,
    auth_token: String,
    user_id: String,
) -> Result<serde_json::Value, String> {
    let state = state.lock().await;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        auth_token.parse().map_err(|_| "Invalid token")?,
    );
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
            .parse()
            .unwrap(),
    );

    // Fetch followers
    let mut all_followers = Vec::new();
    let mut offset = 0;

    loop {
        let url = format!(
            "https://apiv3.fansly.com/api/v1/account/{}/followers?ngsw-bypass=true&limit=100&offset={}",
            user_id, offset
        );

        let response = state
            .client
            .get(&url)
            .headers(headers.clone())
            .send()
            .await
            .map_err(|e| format!("Failed to fetch followers: {}", e))?;

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse followers response: {}", e))?;

        if let Some(followers) = data["response"].as_array() {
            if followers.is_empty() {
                break;
            }
            all_followers.extend(followers.clone());
            offset += 100;
        } else {
            break;
        }
    }

    // Fetch subscribers
    let mut all_subscribers = Vec::new();
    offset = 0;

    loop {
        let url = format!(
            "https://apiv3.fansly.com/api/v1/subscribers?status=3,4&limit=100&offset={}&ngsw-bypass=true",
            offset
        );

        let response = state
            .client
            .get(&url)
            .headers(headers.clone())
            .send()
            .await
            .map_err(|e| format!("Failed to fetch subscribers: {}", e))?;

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse subscribers response: {}", e))?;

        if let Some(subscriptions) = data["response"]["subscriptions"].as_array() {
            if subscriptions.is_empty() {
                break;
            }
            all_subscribers.extend(subscriptions.clone());
            offset += 100;
        } else {
            break;
        }
    }

    let result = serde_json::json!({
        "followers": all_followers,
        "subscribers": all_subscribers,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    Ok(result)
}

#[tauri::command]
async fn fetch_subscription_tiers(
    state: State<'_, Mutex<AppState>>,
    auth_token: String,
    user_id: String,
) -> Result<Vec<SubscriptionTier>, String> {
    let state = state.lock().await;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        auth_token.parse().map_err(|_| "Invalid token")?,
    );
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
            .parse()
            .unwrap(),
    );

    let url = format!(
        "https://apiv3.fansly.com/api/v1/account?ids={}&ngsw-bypass=true",
        user_id
    );

    let response = state
        .client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        let account_response: FanslyAccountResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        if let Some(account) = account_response.response.first() {
            Ok(account.subscription_tiers.clone())
        } else {
            Err("No account data found".to_string())
        }
    } else {
        Err(format!("Fansly API error: {}", response.status()))
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState {
            client: reqwest::Client::new(),
            api_base_url: "http://localhost:1880".to_string(),
        }))
        .invoke_handler(tauri::generate_handler![
            create_account,
            fetch_fansly_data,
            sync_data,
            fetch_followers_and_subscribers,
            fetch_subscription_tiers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
