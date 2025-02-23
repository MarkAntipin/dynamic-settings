use uuid::Uuid;

use chrono::Utc;

use crate::helpers::{create_settings, spawn_app, make_request};
use dynamic_settings::models::{MessageResponse, SettingsDBRow};
use dynamic_settings::enums::SettingsValueType;

#[tokio::test]
async fn test_get_settings_by_key_ok() {
    // Arrange
    let app = spawn_app().await;

    let key = Uuid::new_v4().to_string();
    let value = "100".to_string();

    let settings = SettingsDBRow {
        key: key.clone(),
        value: value.clone(),
        value_type: SettingsValueType::Int,
        created_at: Utc::now(),
    };

    create_settings(&app.partition, &settings);

    // Act
    let response = make_request(
        format!("{}/api/v1/settings/{}", &app.address, key),
        app.api_key.clone(),
        None,
        reqwest::Method::GET,
    ).await;

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
    let key = Uuid::new_v4().to_string();

    // Act
    let response = make_request(
        format!("{}/api/v1/settings/{}", &app.address, key),
        app.api_key.clone(),
        None,
        reqwest::Method::GET,
    ).await;

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
