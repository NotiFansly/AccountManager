// src-tauri/src/api/backend.rs

use tauri::State;
use tokio::sync::Mutex;

use crate::models::{
    AppState, FanslyProfile, SyncDataRequest, SyncDataResponse, VerifyCreatorRequest,
    VerifyCreatorResponse,
};

// --- FIX: Renamed function to match what's in fansly.rs ---
use super::fansly::{
    fetch_followers, fetch_subscribers, fetch_subscription_tiers, get_fansly_profile,
};

/// Command to verify the creator's identity with the backend API and get a sync key.
#[tauri::command]
pub async fn verify_creator_account(
    state: State<'_, Mutex<AppState>>,
    fansly_profile: FanslyProfile,
) -> Result<VerifyCreatorResponse, String> {
    let app_state = state.lock().await;
    let request_payload = VerifyCreatorRequest {
        fansly_profile: &fansly_profile,
    };

    let response = app_state
        .client
        .post(&format!("{}/api/v1/creator/verify", app_state.api_base_url))
        .json(&request_payload)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        response
            .json::<VerifyCreatorResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown API error".to_string());
        Err(error_text)
    }
}

/// Helper function to sync a specific type of data to the backend.
async fn sync_data<T: serde::Serialize>(
    state: State<'_, Mutex<AppState>>,
    sync_key: String,
    data_type: &str,
    data: T,
) -> Result<SyncDataResponse, String> {
    let app_state = state.lock().await;
    let request = SyncDataRequest {
        sync_key,
        data_type: data_type.to_string(),
        data,
    };

    let response = app_state
        .client
        .post(&format!("{}/api/v1/sync", app_state.api_base_url))
        .json(&request)
        .timeout(std::time::Duration::from_secs(60))
        .send()
        .await
        .map_err(|e| format!("Sync request for '{}' failed: {}", data_type, e))?;

    if response.status().is_success() {
        response
            .json::<SyncDataResponse>()
            .await
            .map_err(|e| format!("Failed to parse sync response for '{}': {}", data_type, e))
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown API error".to_string());
        Err(format!("Sync failed for '{}': {}", data_type, error_text))
    }
}

/// The main command called from the UI. Orchestrates fetching all data from Fansly
/// and syncing it to the backend.
#[tauri::command]
pub async fn sync_all_creator_data(
    state: State<'_, Mutex<AppState>>,
    auth_token: String,
    sync_key: String,
) -> Result<Vec<SyncDataResponse>, String> {
    // --- FIX: Call the correctly named function ---
    let profile = get_fansly_profile(state.clone(), auth_token.clone()).await?;
    let user_id = profile.id;

    // Fetch all data types from Fansly concurrently for speed.
    let tiers_handle = fetch_subscription_tiers(state.clone(), auth_token.clone(), user_id.clone());
    let followers_handle = fetch_followers(state.clone(), auth_token.clone(), user_id.clone());
    let subscribers_handle = fetch_subscribers(state.clone(), auth_token.clone());

    let (tiers_res, followers_res, subscribers_res) =
        tokio::join!(tiers_handle, followers_handle, subscribers_handle);

    // Sync each data type to our backend.
    let mut results = Vec::new();

    results.push(
        sync_data(
            state.clone(),
            sync_key.clone(),
            "subscription_tiers",
            tiers_res?,
        )
        .await?,
    );
    results.push(sync_data(state.clone(), sync_key.clone(), "followers", followers_res?).await?);
    results.push(
        sync_data(
            state.clone(),
            sync_key.clone(),
            "subscribers",
            subscribers_res?,
        )
        .await?,
    );

    Ok(results)
}
