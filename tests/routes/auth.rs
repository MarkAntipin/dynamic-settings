use uuid::Uuid;

use crate::helpers::{spawn_app, make_request};
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

    let body: MessageResponse = response.json().await.unwrap();
    assert_eq!(body.message, "missing `X-Api-Key` header");
}

#[tokio::test]
async fn test_auth_invalid_api_key_header() {
    // Arrange
    let app = spawn_app().await;
    let key = Uuid::new_v4().to_string();

    // Act
    let response = make_request(
        format!("{}/api/v1/settings/{}", &app.address, key),
        // not valid api key
        format!("{}:not-valid", app.api_key),
        None,
        reqwest::Method::GET,
    ).await;

    // Assert
    assert_eq!(response.status(), 403);

    let body: MessageResponse = response.json().await.unwrap();
    assert_eq!(body.message, "invalid `X-Api-Key` header");
}


#[tokio::test]
async fn test_validate_token_invalid_token() {
    // Arrange
    let app = spawn_app().await;

    // not valid api key
    let body = serde_json::json!({
        "token": format!("{}:not-valid", app.api_key)
    });

    // Act
    let response = make_request(
        format!("{}/api/v1/auth/validate-token", &app.address),
        app.api_key.clone(),
        Some(body),
        reqwest::Method::POST,
    ).await;

    // Assert
    assert_eq!(response.status(), 403);

    let body: MessageResponse = response.json().await.unwrap();
    assert_eq!(body.message, "Invalid token");
}


#[tokio::test]
async fn test_validate_token_ok() {
    // Arrange
    let app = spawn_app().await;

    // not valid api key
    let body = serde_json::json!({
        "token": app.api_key
    });

    // Act
    let response = make_request(
        format!("{}/api/v1/auth/validate-token", &app.address),
        app.api_key.clone(),
        Some(body),
        reqwest::Method::POST,
    ).await;

    // Assert
    assert_eq!(response.status(), 200);

    let body: MessageResponse = response.json().await.unwrap();
    assert_eq!(body.message, "Token is valid");
}
