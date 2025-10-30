use uuid::Uuid;

use chrono::Utc;
use rand::Rng;
use rand::distr::Alphanumeric;

use crate::helpers::{create_settings, spawn_app, make_request};
use dynamic_settings::models::{SettingsDBRow};
use dynamic_settings::enums::SettingsValueType;

#[tokio::test]
async fn test_get_settings_by_prefix() {
    // Arrange
    let app = spawn_app().await;
    // random prefix
    let mut rng = rand::rng();
    let prefix: String = (0..7).map(|_| rng.sample(Alphanumeric) as char).collect();
    let key = format!("{}{}", prefix, Uuid::new_v4());
    let value = "100".to_string();

    let settings = SettingsDBRow {
        key: key.clone(),
        value: value.clone(),
        value_type: SettingsValueType::Int,
        created_at: Utc::now(),
        updated_at: Utc::now()
    };

    create_settings(&app.pool, &settings).await.expect("Failed to create settings");

    let params = serde_json::json!({
        "prefix": prefix
    });

    // Act
    let response = make_request(
        format!("{}/api/v1/settings", &app.address),
        app.api_key.clone(),
        None,
        Some(params),
        reqwest::Method::GET,
    ).await;

    // Assert
    assert_eq!(response.status(), 200);
    let body: Vec<SettingsDBRow> = response.json().await.unwrap();

    assert_eq!(body.len(), 1);
    assert_eq!(body[0].key, key);
    assert_eq!(body[0].value, value);
}

#[tokio::test]
async fn test_get_settings_no_settings() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = make_request(
        format!("{}/api/v1/settings", &app.address),
        app.api_key.clone(),
        None,
        None,
        reqwest::Method::GET,
    ).await;

    // Assert
    assert_eq!(response.status(), 200);
    let body: Vec<SettingsDBRow> = response.json().await.unwrap();

    assert_eq!(body.len(), 0);
}
