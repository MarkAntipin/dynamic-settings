use uuid::Uuid;

use crate::helpers::spawn_app;
use dynamic_settings::models::{MessageResponse, Settings, SettingsValueType};
use dynamic_settings::repository::pg_add_settings;

#[tokio::test]
async fn test_get_settings_by_key_ok() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let key = Uuid::new_v4().to_string();
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

    let response = client
        .get(&format!("{}/api/v1/settings/{}", &app.address, key))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 200);

    let body: Settings = response
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

    // Act
    let response = client
        .get(&format!("{}/api/v1/settings/{}", &app.address, key))
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
