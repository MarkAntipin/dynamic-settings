use uuid::Uuid;

use chrono::Utc;

use crate::helpers::{create_settings, get_settings, spawn_app, make_request};
use dynamic_settings::models::{SettingsDBRow};
use dynamic_settings::enums::SettingsValueType;

#[tokio::test]
async fn test_deleted_settings() {
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

    create_settings(&app.partition, &settings);

    let body = serde_json::json!({
        "keys": [key]
    });

    // Act
    let response = make_request(
        format!("{}/api/v1/settings", &app.address),
        app.api_key.clone(),
        Some(body),
        reqwest::Method::DELETE,
    ).await;

    // Assert
    assert_eq!(response.status(), 200);

    let result = get_settings(&app.partition, &key);
    assert!(result.unwrap().is_none());
}
