use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use tauri::State;
use tokio::sync::Mutex;

use crate::models::{
    AccountDetailsResponse, AppState, FanslyFollower, FanslyFollowerResponse, FanslyProfile,
    FanslyProfileResponse, FanslySubscriber, FanslySubscriberResponse, SubscriptionTier,
};

fn create_fansly_headers(auth_token: &str) -> Result<HeaderMap, String> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        auth_token.parse().map_err(|_| "Invalid token format")?,
    );
    headers.insert(USER_AGENT, "NotiFansly-Sync/1.0".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    Ok(headers)
}

#[tauri::command]
pub async fn get_fansly_profile(
    state: State<'_, Mutex<AppState>>,
    auth_token: String,
) -> Result<FanslyProfile, String> {
    let app_state = state.lock().await;
    let headers = create_fansly_headers(&auth_token)?;

    let response = app_state
        .client
        .get("https://apiv3.fansly.com/api/v1/account/me?ngsw-bypass=true")
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        let data = response
            .json::<FanslyProfileResponse>()
            .await
            .map_err(|e| format!("Failed to parse profile response: {}", e))?;
        Ok(data.response.account)
    } else {
        Err(format!("Fansly API error: {}", response.status()))
    }
}

pub async fn fetch_subscription_tiers(
    state: State<'_, Mutex<AppState>>,
    auth_token: String,
    user_id: String,
) -> Result<Vec<SubscriptionTier>, String> {
    let app_state = state.lock().await;
    let headers = create_fansly_headers(&auth_token)?;
    let url = format!(
        "https://apiv3.fansly.com/api/v1/account?ids={}&ngsw-bypass=true",
        user_id
    );

    let response = app_state
        .client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        let data = response
            .json::<AccountDetailsResponse>()
            .await
            .map_err(|e| format!("Failed to parse tiers response: {}", e))?;
        Ok(data
            .response
            .into_iter()
            .next()
            .map_or(vec![], |acc| acc.subscription_tiers))
    } else {
        Err(format!(
            "Fansly API error getting tiers: {}",
            response.status()
        ))
    }
}

pub async fn fetch_followers(
    state: State<'_, Mutex<AppState>>,
    auth_token: String,
    user_id: String,
) -> Result<Vec<FanslyFollower>, String> {
    let app_state = state.lock().await;
    let headers = create_fansly_headers(&auth_token)?;
    let mut all_followers = Vec::new();
    let mut offset = 0;

    loop {
        let url = format!(
            "https://apiv3.fansly.com/api/v1/account/{}/followers?limit=100&offset={}",
            user_id, offset
        );
        let response = app_state
            .client
            .get(&url)
            .headers(headers.clone())
            .send()
            .await
            .map_err(|e| format!("Follower request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Fansly API error getting followers: {}",
                response.status()
            ));
        }

        let data = response
            .json::<FanslyFollowerResponse>()
            .await
            .map_err(|e| format!("Failed to parse followers response: {}", e))?;

        if data.response.is_empty() {
            break;
        }
        let count = data.response.len();
        all_followers.extend(data.response);
        offset += count;
        if count < 100 {
            break;
        }
    }
    Ok(all_followers)
}

pub async fn fetch_subscribers(
    state: State<'_, Mutex<AppState>>,
    auth_token: String,
) -> Result<Vec<FanslySubscriber>, String> {
    let app_state = state.lock().await;
    let headers = create_fansly_headers(&auth_token)?;
    let mut all_subscribers = Vec::new();
    let mut offset = 0;

    loop {
        // status=3 (active), status=4 (lapsed but still has access)
        let url = format!(
            "https://apiv3.fansly.com/api/v1/subscribers?status=3,4&limit=100&offset={}",
            offset
        );
        let response = app_state
            .client
            .get(&url)
            .headers(headers.clone())
            .send()
            .await
            .map_err(|e| format!("Subscriber request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Fansly API error getting subscribers: {}",
                response.status()
            ));
        }

        let data = response
            .json::<FanslySubscriberResponse>()
            .await
            .map_err(|e| format!("Failed to parse subscribers response: {}", e))?;

        if data.response.subscriptions.is_empty() {
            break;
        }
        let count = data.response.subscriptions.len();
        all_subscribers.extend(data.response.subscriptions);
        offset += count;
        if count < 100 {
            break;
        }
    }
    Ok(all_subscribers)
}
