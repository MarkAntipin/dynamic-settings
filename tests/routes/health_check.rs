use crate::helpers::{spawn_app, make_request};

#[tokio::test]
async fn test_health_check_ok() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = make_request(
        format!("{}/health", &app.address),
        app.api_key.clone(),
        None,
        reqwest::Method::GET,
    ).await;

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
