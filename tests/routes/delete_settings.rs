use uuid::Uuid;

use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::helpers::{create_settings, get_settings, spawn_app};
use dynamic_settings::models::{SettingsDBRow};
use dynamic_settings::enums::SettingsValueType;

#[tokio::test]
async fn test_deleted_settings() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let key = Uuid::new_v4().to_string();
    let value = "100".to_string();

    let settings = SettingsDBRow {
        key: key.clone(),
        value: value.clone(),
        value_type: SettingsValueType::Int,
        created_at: Utc::now()
    };

    create_settings(&app.partition, &settings);

    let body = serde_json::json!({
        "keys": [key]
    });


    let mut headers = HeaderMap::new();
    headers.insert(
        "X-Api-Key",
        HeaderValue::from_str(&app.api_key).expect("Failed to add header"),
    );

    // Act
    let response = client
        .delete(format!("{}/api/v1/settings", &app.address))
        .headers(headers)
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 200);

    let result = get_settings(&app.partition, &key);
    assert!(result.unwrap().is_none());
}
