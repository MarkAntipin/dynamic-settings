use uuid::Uuid;

use crate::helpers::spawn_app;
use dynamic_settings::models::MessageResponse;
use dynamic_settings::models::{Settings, SettingsValueType};
use dynamic_settings::repository::{pg_add_settings, pg_get_settings_by_key};

#[tokio::test]
async fn test_add_settings_int_ok() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let key = Uuid::new_v4().to_string();
    let value_type = "int";
    let value = "100";

    // Act
    let body = serde_json::json!({
        "key": key,
        "value": value,
        "type": value_type
    });
    let response = client
        .post(&format!("{}/api/v1/settings", &app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 201);

    let body: MessageResponse = response
        .json()
        .await
        .expect("Failed to parse response body.");
    assert_eq!(body.message, format!("Settings with key '{}' created", key));

    // TODO: is it ok to use func from repository directly in tests?
    let settings = pg_get_settings_by_key(&app.pg_pool, &key)
        .await
        .expect("Failed to fetch the setting")
        .expect(&format!("Settings not found for key: {}", key));

    assert_eq!(settings.key, key);
    assert_eq!(settings.value, value);
}

#[tokio::test]
async fn test_add_settings_float_ok() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let key = Uuid::new_v4().to_string();
    let value_type = "float";
    let value = "100.5";

    // Act
    let body = serde_json::json!({
        "key": key,
        "value": value,
        "type": value_type
    });
    let response = client
        .post(&format!("{}/api/v1/settings", &app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    println!("{:?}", response);
    assert_eq!(response.status(), 201);

    let body: MessageResponse = response
        .json()
        .await
        .expect("Failed to parse response body.");
    assert_eq!(body.message, format!("Settings with key '{}' created", key));

    let settings = pg_get_settings_by_key(&app.pg_pool, &key)
        .await
        .expect("Failed to fetch the setting")
        .expect(&format!("Settings not found for key: {}", key));

    assert_eq!(settings.key, key);
    assert_eq!(settings.value, value);
}

#[tokio::test]
async fn test_add_settings_bool_ok() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let key = Uuid::new_v4().to_string();
    let value_type = "bool";
    let value = "true";

    // Act
    let body = serde_json::json!({
        "key": key,
        "value": value,
        "type": value_type
    });
    let response = client
        .post(&format!("{}/api/v1/settings", &app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 201);

    let body: MessageResponse = response
        .json()
        .await
        .expect("Failed to parse response body.");
    assert_eq!(body.message, format!("Settings with key '{}' created", key));

    let settings = pg_get_settings_by_key(&app.pg_pool, &key)
        .await
        .expect("Failed to fetch the setting")
        .expect(&format!("Settings not found for key: {}", key));

    assert_eq!(settings.key, key);
    assert_eq!(settings.value, value);
}

#[tokio::test]
async fn test_add_settings_int_invalid() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let key = Uuid::new_v4().to_string();
    let value_type = "int";
    let value = "invalid";

    // Act
    let body = serde_json::json!({
        "key": key,
        "value": value,
        "type": value_type
    });
    let response = client
        .post(&format!("{}/api/v1/settings", &app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 422);

    let body: MessageResponse = response
        .json()
        .await
        .expect("Failed to parse response body.");
    assert_eq!(body.message, "Value 'invalid' is not a valid integer");
}

#[tokio::test]
async fn test_add_settings_key_already_exists() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let key = Uuid::new_v4().to_string();
    let value_type = "int";
    let value = "100".to_string();

    let settings = Settings {
        key: key.clone(),
        value: value.clone(),
        value_type: SettingsValueType::Int,
    };

    pg_add_settings(&app.pg_pool, &settings)
        .await
        .expect("Failed to add settings");

    // Act
    let body = serde_json::json!({
        "key": key,
        "value": value,
        "type": value_type
    });

    let response = client
        .post(&format!("{}/api/v1/settings", &app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

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
async fn test_add_settings_invalid_input() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let key = Uuid::new_v4().to_string();
    let value = "invalid";

    // Act
    let body = serde_json::json!({
        "key": key,
        "value": value,
        // missing type
    });
    let response = client
        .post(&format!("{}/api/v1/settings", &app.address))
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 422);

    let body: MessageResponse = response
        .json()
        .await
        .expect("Failed to parse response body.");
    assert_eq!(body.message, "Json deserialize error: missing field `type`");
}
