use serde_json;
use tauri::State;
use tokio::sync::Mutex;

use crate::models::{
    AppState,
    CreateAccountRequest,
    CreateAccountResponse,
    //    SubscriptionTier, // Keep this, as it's used when serializing subscription_tiers
    SyncDataRequest,
    SyncResponse,
};

// Import the Fansly API functions
use super::fansly::{fetch_followers_and_subscribers, fetch_subscription_tiers};

#[tauri::command]
pub async fn create_account(
    state: State<'_, Mutex<AppState>>,
    fansly_profile: serde_json::Value,
) -> Result<CreateAccountResponse, String> {
    let state = state.lock().await;

    // Create the new request struct, wrapping the profile data.
    let request = CreateAccountRequest { fansly_profile };

    let response = state
        .client
        .post(&format!("{}/api/v1/accounts", state.api_base_url))
        .json(&request) // reqwest will serialize our struct into the correct JSON payload.
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
        // This will now correctly return the server's validation error
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown API error".to_string());
        Err(format!("{}", error_text)) // Just return the error message directly
    }
}

#[tauri::command]
pub async fn sync_data(
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
        sync_mode: "replace".to_string(),
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
pub async fn sync_data_enhanced(
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
        sync_mode: "replace".to_string(),
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
pub async fn sync_all_data(
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
