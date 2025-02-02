use uuid::Uuid;

use reqwest::header::{HeaderMap, HeaderValue};

use crate::helpers::spawn_app;
use dynamic_settings::models::MessageResponse;

#[tokio::test]
async fn test_auth_no_api_key_header() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let key = Uuid::new_v4().to_string();

    // Act
    let response = client
        .get(format!("{}/api/v1/settings/{}", &app.address, key))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 401);

    let body: MessageResponse = response
        .json()
        .await
        .expect("Failed to parse response body.");

    assert_eq!(body.message, "missing `X-Api-Key` header");
}

#[tokio::test]
async fn test_auth_invalid_api_key_header() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let key = Uuid::new_v4().to_string();

    let mut headers = HeaderMap::new();
    // not valid api key
    headers.insert(
        "X-Api-Key",
        HeaderValue::from_str(&format!("{}:not-valid", app.api_key)).expect("Failed to add header"),
    );

    // Act
    let response = client
        .get(format!("{}/api/v1/settings/{}", &app.address, key))
        .headers(headers)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 403);

    let body: MessageResponse = response
        .json()
        .await
        .expect("Failed to parse response body.");

    assert_eq!(body.message, "invalid `X-Api-Key` header");
}
