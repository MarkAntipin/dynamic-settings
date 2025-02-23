use uuid::Uuid;

use chrono::Utc;

use crate::helpers::{create_settings, get_settings, spawn_app, make_request};
use dynamic_settings::models::MessageResponse;
use dynamic_settings::models::SettingsDBRow;
use dynamic_settings::enums::SettingsValueType;


#[tokio::test]
async fn test_create_settings_ok() {
    // TODO: use some kind of `parameterized` here
    let test_cases = vec![
        ("100", "int"),
        ("100.5", "float"),
        ("true", "bool"),
        ("\"string\"", "str"),
        ("[1, 2, 3]", "json"),
        ("{\"key\": \"value\", \"key2\": [1, 2, 3]}", "json"),
    ];

    // Arrange
    let app = spawn_app().await;

    for (value, value_type) in test_cases {
        // Arrange
        let key = Uuid::new_v4().to_string();

        let body = serde_json::json!({
            "key": key,
            "value": value,
            "type": value_type
        });

        // Act
        let response = make_request(
            format!("{}/api/v1/settings", &app.address),
            app.api_key.clone(),
            Some(body),
            reqwest::Method::POST,
        ).await;

        // Assert
        assert_eq!(response.status(), 201);

        let body: MessageResponse = response
            .json()
            .await
            .expect("Failed to parse response body.");
        assert_eq!(body.message, format!("Settings with key '{}' created", key));

        let settings = get_settings(&app.partition, &key)
            .expect("Failed to fetch the setting")
            .unwrap_or_else(|| panic!("Settings not found for key: {}", key));

        assert_eq!(settings.key, key);
        assert_eq!(settings.value, value);
    }
}

#[tokio::test]
async fn test_create_settings_int_invalid() {
    // Arrange
    let app = spawn_app().await;

    let key = Uuid::new_v4().to_string();
    let value_type = "int";
    let value = "invalid";

    let body = serde_json::json!({
        "key": key,
        "value": value,
        "type": value_type
    });

    // Act
    let response = make_request(
        format!("{}/api/v1/settings", &app.address),
        app.api_key.clone(),
        Some(body),
        reqwest::Method::POST,
    ).await;

    // Assert
    assert_eq!(response.status(), 422);

    let body: MessageResponse = response
        .json()
        .await
        .expect("Failed to parse response body.");
    assert_eq!(body.message, "Value 'invalid' is not a valid integer");
}

#[tokio::test]
async fn test_create_settings_key_already_exists() {
    // Arrange
    let app = spawn_app().await;

    let key = Uuid::new_v4().to_string();
    let value_type = "int";
    let value = "100".to_string();

    let settings = SettingsDBRow {
        key: key.clone(),
        value: value.clone(),
        value_type: SettingsValueType::Int,
        created_at: Utc::now(),
    };

    create_settings(&app.partition, &settings);

    let body = serde_json::json!({
        "key": key,
        "value": value,
        "type": value_type
    });

    // Act
    let response = make_request(
        format!("{}/api/v1/settings", &app.address),
        app.api_key.clone(),
        Some(body),
        reqwest::Method::POST,
    ).await;

    // Assert
    assert_eq!(response.status(), 409);

    let body: MessageResponse = response
        .json()
        .await
        .expect("Failed to parse response body.");
    assert_eq!(
        body.message,
        format!("Settings with key '{}' already exist", key)
    );
}

#[tokio::test]
async fn test_create_settings_invalid_input_missing_type() {
    // Arrange
    let app = spawn_app().await;

    let key = Uuid::new_v4().to_string();
    let value = "invalid";

    let body = serde_json::json!({
        "key": key,
        "value": value,
        // missing type
    });

    // Act
    let response = make_request(
        format!("{}/api/v1/settings", &app.address),
        app.api_key.clone(),
        Some(body),
        reqwest::Method::POST,
    ).await;

    // Assert
    assert_eq!(response.status(), 422);

    let body: MessageResponse = response
        .json()
        .await
        .expect("Failed to parse response body.");
    assert_eq!(body.message, "Json deserialize error: missing field `type`");
}

#[tokio::test]
async fn test_create_settings_invalid_input_key_is_to_big() {
    // Arrange
    let app = spawn_app().await;

    let value_type = "int";
    let value = "100";

    let body = serde_json::json!({
        "key": "a".repeat(10000),
        "value": value,
        "type": value_type
    });

    // Act
    let response = make_request(
        format!("{}/api/v1/settings", &app.address),
        app.api_key.clone(),
        Some(body),
        reqwest::Method::POST,
    ).await;

    // Assert
    assert_eq!(response.status(), 422);

    let body: MessageResponse = response
        .json()
        .await
        .expect("Failed to parse response body.");
    assert_eq!(body.message, "Key length should be less than 1024 bytes");
}
