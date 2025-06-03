use chrono::{DateTime, Utc};
use reqwest;
use std::collections::HashMap;
use tauri::State;
use tokio::sync::Mutex;

// Import all necessary types from the models module using absolute path
use crate::models::{
    AppState,
    FanslyAccountData,
    FanslyAccountResponse,
    //FanslyFollower,
    FanslyFollowerResponse,
    FanslyResponse,
    //FanslyResponseData,
    //FanslySubscriber,
    //FanslySubscriberData,
    FanslySubscriberResponse,
    SubscriptionTier,
};

#[tauri::command]
pub async fn fetch_fansly_data(
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
pub async fn fetch_followers_and_subscribers(
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
pub async fn fetch_subscription_tiers(
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
