use uuid::Uuid;

use chrono::Utc;

use crate::helpers::{create_settings, get_settings, spawn_app, make_request};
use dynamic_settings::models::{MessageResponse, SettingsDBRow};
use dynamic_settings::enums::SettingsValueType;

#[tokio::test]
async fn test_delete_settings() {
    // Arrange
    let app = spawn_app().await;

    let key = Uuid::new_v4().to_string();
    let value = "100".to_string();

    let settings = SettingsDBRow {
        key: key.clone(),
        value: value.clone(),
        value_type: SettingsValueType::Int,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    create_settings(&app.pool, &settings).await.expect("Failed to create settings");

    let body = serde_json::json!({
        "keys": [key]
    });

    // Act
    let response = make_request(
        format!("{}/api/v1/settings", &app.address),
        app.api_key.clone(),
        Some(body),
        None,
        reqwest::Method::DELETE,
    ).await;

    // Assert
    assert_eq!(response.status(), 200);

    let settings = get_settings(&app.pool, &key).await.unwrap();
    assert!(settings.is_none());
}

#[tokio::test]
async fn test_delete_settings_not_exist() {
    // Arrange
    let app = spawn_app().await;

    let key = Uuid::new_v4().to_string();

    let body = serde_json::json!({
        "keys": [key]
    });

    // Act
    let response = make_request(
        format!("{}/api/v1/settings", &app.address),
        app.api_key.clone(),
        Some(body),
        None,
        reqwest::Method::DELETE,
    ).await;

    // Assert
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_delete_settings_no_keys_provided() {
    // Arrange
    let app = spawn_app().await;

    let body = serde_json::json!({
        "keys": []
    });

    // Act
    let response = make_request(
        format!("{}/api/v1/settings", &app.address),
        app.api_key.clone(),
        Some(body),
        None,
        reqwest::Method::DELETE,
    ).await;

    // Assert
    assert_eq!(response.status(), 422);

    let body: MessageResponse = response.json().await.unwrap();
    assert_eq!(body.message, "Keys array should not be empty");
}
