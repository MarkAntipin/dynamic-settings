use uuid::Uuid;

use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::helpers::{create_settings, spawn_app};
use dynamic_settings::models::{MessageResponse, SettingsDBRow};
use dynamic_settings::enums::SettingsValueType;

#[tokio::test]
async fn test_get_settings_by_key_ok() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let key = Uuid::new_v4().to_string();
    let value = "100".to_string();

    let settings = SettingsDBRow {
        key: key.clone(),
        value: value.clone(),
        value_type: SettingsValueType::Int,
        created_at: Utc::now(),
    };

    create_settings(&app.partition, &settings);

    let mut headers = HeaderMap::new();
    headers.insert(
        "X-Api-Key",
        HeaderValue::from_str(&app.api_key).expect("Failed to add header"),
    );

    // Act
    let response = client
        .get(format!("{}/api/v1/settings/{}", &app.address, key))
        .headers(headers)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 200);

    let body: SettingsDBRow = response
        .json()
        .await
        .expect("Failed to parse response body.");
    assert_eq!(body.key, key);
    assert_eq!(body.value, value);
}

#[tokio::test]
async fn test_get_settings_by_key_key_does_not_exist() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let key = Uuid::new_v4().to_string();

    let mut headers = HeaderMap::new();
    headers.insert(
        "X-Api-Key",
        HeaderValue::from_str(&app.api_key).expect("Failed to add header"),
    );

    // Act
    let response = client
        .get(format!("{}/api/v1/settings/{}", &app.address, key))
        .headers(headers)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 404);

    let body: MessageResponse = response
        .json()
        .await
        .expect("Failed to parse response body.");
    assert_eq!(
        body.message,
        format!("Settings with key '{}' not found", key)
    );
}
