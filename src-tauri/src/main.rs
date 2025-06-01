// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{DateTime, Utc};
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, State, WindowEvent,
};
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

#[derive(Debug, Serialize, Deserialize)]
struct FanslyFollowerResponse {
    success: bool,
    response: Vec<FanslyFollower>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FanslyFollower {
    #[serde(rename = "followerId")]
    follower_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FanslySubscriberResponse {
    success: bool,
    response: FanslySubscriberData,
}

#[derive(Debug, Serialize, Deserialize)]
struct FanslySubscriberData {
    subscriptions: Vec<FanslySubscriber>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FanslySubscriber {
    id: String,
    #[serde(rename = "historyId")]
    history_id: String,
    #[serde(rename = "subscriberId")]
    subscriber_id: String,
    #[serde(rename = "subscriptionTierId")]
    subscription_tier_id: String,
    #[serde(rename = "subscriptionTierName")]
    subscription_tier_name: String,
    #[serde(rename = "subscriptionTierColor")]
    subscription_tier_color: String,
    #[serde(rename = "planId")]
    plan_id: String,
    #[serde(rename = "promoId")]
    promo_id: Option<String>,
    #[serde(rename = "giftCodeId")]
    gift_code_id: Option<String>,
    #[serde(rename = "paymentMethodId")]
    payment_method_id: String,
    status: i32,
    price: i32,
    #[serde(rename = "renewPrice")]
    renew_price: i32,
    #[serde(rename = "renewCorrelationId")]
    renew_correlation_id: String,
    #[serde(rename = "autoRenew")]
    auto_renew: i32,
    #[serde(rename = "billingCycle")]
    billing_cycle: i32,
    duration: i32,
    #[serde(rename = "renewDate")]
    renew_date: i64,
    version: i32,
    #[serde(rename = "createdAt")]
    created_at: i64,
    #[serde(rename = "updatedAt")]
    updated_at: i64,
    #[serde(rename = "endsAt")]
    ends_at: i64,
    #[serde(rename = "promoPrice")]
    promo_price: Option<i32>,
    #[serde(rename = "promoDuration")]
    promo_duration: Option<i32>,
    #[serde(rename = "promoStatus")]
    promo_status: Option<i32>,
    #[serde(rename = "promoStartsAt")]
    promo_starts_at: Option<i64>,
    #[serde(rename = "promoEndsAt")]
    promo_ends_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SyncResponse {
    success: bool,
    message: String,
    data_type: String,
    count: i32,
    timestamp: String,
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
async fn sync_data_enhanced(
    state: State<'_, Mutex<AppState>>,
    sync_key: String,
    data_type: String,
    data: serde_json::Value,
) -> Result<SyncResponse, String> {
    let state = state.lock().await;

    println!(
        "Syncing {} data with {} items",
        data_type,
        data.as_array().map(|arr| arr.len()).unwrap_or(0)
    );

    let request = SyncDataRequest {
        sync_key: sync_key.clone(),
        data_type: data_type.clone(),
        data,
    };

    let response = state
        .client
        .post(&format!("{}/api/v1/sync", state.api_base_url))
        .json(&request)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = response.status();
    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    println!("Sync API response status: {}", status);
    println!("Sync API response body: {}", response_text);

    if status.is_success() {
        let sync_response: SyncResponse = serde_json::from_str(&response_text).map_err(|e| {
            format!(
                "Failed to parse sync response: {} - Response: {}",
                e, response_text
            )
        })?;

        println!(
            "Sync successful for {}: {}",
            data_type, sync_response.message
        );
        Ok(sync_response)
    } else {
        Err(format!("Sync failed ({}): {}", status, response_text))
    }
}

// Batch sync function for multiple data types
#[tauri::command]
async fn sync_all_data(
    state: State<'_, Mutex<AppState>>,
    sync_key: String,
    auth_token: String,
    user_id: String,
) -> Result<Vec<SyncResponse>, String> {
    let mut results = Vec::new();

    // Fetch all data types
    let followers_and_subscribers =
        fetch_followers_and_subscribers(state.clone(), auth_token.clone(), user_id.clone()).await?;

    let subscription_tiers =
        fetch_subscription_tiers(state.clone(), auth_token.clone(), user_id.clone()).await?;

    // Sync followers
    if let Some(followers) = followers_and_subscribers.get("followers") {
        match sync_data_enhanced(
            state.clone(),
            sync_key.clone(),
            "followers".to_string(),
            followers.clone(),
        )
        .await
        {
            Ok(response) => results.push(response),
            Err(e) => return Err(format!("Failed to sync followers: {}", e)),
        }
    }

    // Sync subscribers
    if let Some(subscribers) = followers_and_subscribers.get("subscribers") {
        match sync_data_enhanced(
            state.clone(),
            sync_key.clone(),
            "subscribers".to_string(),
            subscribers.clone(),
        )
        .await
        {
            Ok(response) => results.push(response),
            Err(e) => return Err(format!("Failed to sync subscribers: {}", e)),
        }
    }

    // Sync subscription tiers
    match sync_data_enhanced(
        state.clone(),
        sync_key.clone(),
        "subscription_tiers".to_string(),
        serde_json::to_value(subscription_tiers)
            .map_err(|e| format!("Failed to serialize tiers: {}", e))?,
    )
    .await
    {
        Ok(response) => results.push(response),
        Err(e) => return Err(format!("Failed to sync subscription tiers: {}", e)),
    }

    Ok(results)
}

#[tauri::command]
async fn fetch_followers_and_subscribers(
    state: State<'_, Mutex<AppState>>,
    auth_token: String,
    user_id: String,
) -> Result<HashMap<String, serde_json::Value>, String> {
    // Changed return type
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

    // Fetch followers with better error handling
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

        // Debug the response
        let status = response.status();
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        println!("Followers API response status: {}", status);
        println!("Followers API response body: {}", response_text);

        if !status.is_success() {
            return Err(format!("Followers API error {}: {}", status, response_text));
        }

        // Try to parse the response
        let follower_response: FanslyFollowerResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                format!(
                    "Failed to parse followers response: {} - Response: {}",
                    e, response_text
                )
            })?;

        if follower_response.response.is_empty() {
            break;
        }

        let response_len = follower_response.response.len();
        for follower in &follower_response.response {
            all_followers.push(serde_json::json!({
                "id": follower.follower_id,
                "followedAt": Utc::now().to_rfc3339()
            }));
        }

        offset += 100;
        if response_len < 100 {
            break;
        }
    }

    // Fetch subscribers with better error handling - KEEP YOUR STATUS FILTER
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

        let status = response.status();
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        println!("Subscribers API response status: {}", status);
        println!("Subscribers API response body: {}", response_text);

        if !status.is_success() {
            return Err(format!(
                "Subscribers API error {}: {}",
                status, response_text
            ));
        }

        let subscriber_response: FanslySubscriberResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                format!(
                    "Failed to parse subscribers response: {} - Response: {}",
                    e, response_text
                )
            })?;

        if subscriber_response.response.subscriptions.is_empty() {
            break;
        }

        let subscriptions_len = subscriber_response.response.subscriptions.len();
        for sub in &subscriber_response.response.subscriptions {
            all_subscribers.push(serde_json::json!({
                "id": sub.id,
                "accountId": sub.subscriber_id,
                "subscriptionTierId": sub.subscription_tier_id,
                "status": sub.status,
                "createdAt": DateTime::from_timestamp(sub.created_at / 1000, 0)
                    .unwrap_or_default()
                    .to_rfc3339(),
                "endsAt": DateTime::from_timestamp(sub.ends_at / 1000, 0)
                    .unwrap_or_default()
                    .to_rfc3339(),
                "price": sub.price,
                "billingCycle": sub.billing_cycle,
                "autoRenew": sub.auto_renew == 1
            }));
        }

        offset += 100;
        if subscriptions_len < 100 {
            break;
        }
    }

    // Return as HashMap for sync_all_data compatibility
    let mut result = HashMap::new();
    result.insert(
        "followers".to_string(),
        serde_json::Value::Array(all_followers),
    );
    result.insert(
        "subscribers".to_string(),
        serde_json::Value::Array(all_subscribers),
    );

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

    let status = response.status();
    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    println!("Subscription tiers API response status: {}", status);
    println!("Subscription tiers API response body: {}", response_text);

    if !status.is_success() {
        return Err(format!("Fansly API error {}: {}", status, response_text));
    }

    let account_response: FanslyAccountResponse =
        serde_json::from_str(&response_text).map_err(|e| {
            format!(
                "Failed to parse response: {} - Response: {}",
                e, response_text
            )
        })?;

    if let Some(account) = account_response.response.first() {
        Ok(account.subscription_tiers.clone())
    } else {
        Err("No account data found".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState {
            client: reqwest::Client::new(),
            api_base_url: "http://localhost:1880".to_string(),
        }))
        .setup(|app| {
            // Create menu items with correct syntax
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
            let sync_now = MenuItem::with_id(app, "sync_now", "Sync Now", true, None::<&str>)?;
            let toggle_auto_sync = MenuItem::with_id(
                app,
                "toggle_auto_sync",
                "Toggle Auto Sync",
                true,
                None::<&str>,
            )?;

            let menu = Menu::with_items(app, &[&show, &hide, &sync_now, &toggle_auto_sync, &quit])?;

            // Create system tray
            let _tray = TrayIconBuilder::with_id("main-tray")
                .tooltip("Fansly Sync App")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        let windows = app.webview_windows();
                        if let Some(window) = windows.values().next() {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        let windows = app.webview_windows();
                        if let Some(window) = windows.values().next() {
                            let _ = window.hide();
                        }
                    }
                    "sync_now" => {
                        // Trigger sync via event
                        let windows = app.webview_windows();
                        if let Some(window) = windows.values().next() {
                            let _ = window.emit("tray-sync-now", ());
                        }
                    }
                    "toggle_auto_sync" => {
                        // Toggle auto sync via event
                        let windows = app.webview_windows();
                        if let Some(window) = windows.values().next() {
                            let _ = window.emit("tray-toggle-auto-sync", ());
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(webview_window) = app.get_webview_window("main") {
                            // Toggle window visibility on left click
                            if webview_window.is_visible().unwrap_or(false) {
                                let _ = webview_window.hide();
                            } else {
                                let _ = webview_window.show();
                                let _ = webview_window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                WindowEvent::CloseRequested { api, .. } => {
                    // Hide the window instead of closing it
                    window.hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            create_account,
            fetch_fansly_data,
            sync_data,
            sync_data_enhanced,
            sync_all_data,
            fetch_followers_and_subscribers,
            fetch_subscription_tiers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
