use uuid::Uuid;

use chrono::Utc;

use crate::helpers::{create_settings, spawn_app, make_request, get_settings};
use dynamic_settings::models::{MessageResponse, SettingsDBRow};
use dynamic_settings::enums::SettingsValueType;

#[tokio::test]
async fn test_update_settings_ok() {
    // Arrange
    let app = spawn_app().await;

    let key = Uuid::new_v4().to_string();

    let now = Utc::now();
    let settings = SettingsDBRow {
        key: key.clone(),
        value: "100".to_string(),
        value_type: SettingsValueType::Int,
        created_at: now,
        updated_at: now,
    };
    create_settings(&app.partition, &settings);
    let body = serde_json::json!({
        "key": key,
        "value": "200",
    });

    // Act
    let response = make_request(
        format!("{}/api/v1/settings", &app.address),
        app.api_key.clone(),
        Some(body),
        reqwest::Method::PUT,
    ).await;

    // Assert
    assert_eq!(response.status(), 200);

    let body: MessageResponse = response.json().await.unwrap();
    assert_eq!(body.message, "Settings updated");

    let settings = get_settings(&app.partition, &key).unwrap().unwrap();
    assert_eq!(settings.key, key);
    assert_eq!(settings.value, "200");
    assert!(settings.updated_at > now);
}

#[tokio::test]
async fn test_update_settings_key_does_not_exist() {
    // Arrange
    let app = spawn_app().await;
    let key = Uuid::new_v4().to_string();

    let body = serde_json::json!({
        "key": key,
        "value": "200",
    });
    // Act
    let response = make_request(
        format!("{}/api/v1/settings", &app.address),
        app.api_key.clone(),
        Some(body),
        reqwest::Method::PUT,
    ).await;

    // Assert
    assert_eq!(response.status(), 404);

    let body: MessageResponse = response.json().await.unwrap();
    assert_eq!(
        body.message,
        format!("Settings with key '{}' not found", key)
    );
}

#[tokio::test]
async fn test_update_settings_invalid_input_invalid_type() {
    // Arrange
    let app = spawn_app().await;

    let key = Uuid::new_v4().to_string();

    let now = Utc::now();
    let settings = SettingsDBRow {
        key: key.clone(),
        value: "100".to_string(),
        value_type: SettingsValueType::Int,
        created_at: now,
        updated_at: now,
    };
    create_settings(&app.partition, &settings);
    let body = serde_json::json!({
        "key": key,
        "value": "Not int value",
    });

    // Act
    let response = make_request(
        format!("{}/api/v1/settings", &app.address),
        app.api_key.clone(),
        Some(body),
        reqwest::Method::PUT,
    ).await;

    // Assert
    assert_eq!(response.status(), 422);

    let body: MessageResponse = response.json().await.unwrap();
    assert_eq!(body.message, "Value 'Not int value' is not a valid integer");

    let settings = get_settings(&app.partition, &key).unwrap().unwrap();
    assert_eq!(settings.key, key);
    assert_eq!(settings.value, "100");
    assert_eq!(settings.created_at, now);
}