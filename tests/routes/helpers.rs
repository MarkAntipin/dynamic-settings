use std::option::Option;
use std::path::PathBuf;
use std::fs;
use uuid::Uuid;

use dynamic_settings::config::get_config;
use dynamic_settings::models::{SettingsDBRow, SettingsDB};
use dynamic_settings::startup;
use std::net::TcpListener;
use reqwest::header::{HeaderMap, HeaderValue};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub struct TestApp {
    pub address: String,
    pub pool: SqlitePool,
    pub api_key: String,
    pub db_path: PathBuf,
}

impl Drop for TestApp {
    fn drop(&mut self) {
        // Clean up the database file after tests
        if let Err(e) = fs::remove_file(&self.db_path) {
            eprintln!("Failed to remove test database file: {}", e);
        }
    }
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let config = get_config().expect("Failed to read configuration.");

    // Create SQLite database
    let db_path = PathBuf::from(format!("test_db_{}.db", Uuid::new_v4()));
    let db_url = format!("sqlite://{}", db_path.to_string_lossy());

    fs::File::create(&db_path).expect("Failed to create test database file");

    let pool = SqlitePoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to create SQLite pool");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let settings_db = SettingsDB {
        pool: pool.clone(),
    };

    let server = startup::run(listener, settings_db, config.api_key.clone())
        .expect("Failed to bind address");

    tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        pool,
        api_key: config.api_key,
        db_path,
    }
}

pub async fn make_request(
    url: String,
    api_key: String,
    body: Option<serde_json::Value>,
    params: Option<serde_json::Value>,
    method: reqwest::Method,
) -> reqwest::Response {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();

    headers.insert(
        "X-Api-Key",
        HeaderValue::from_str(&api_key).expect("Failed to add header"),
    );
    let request_builder = client
        .request(method, url)
        .headers(headers);

    let request_builder = if let Some(json_body) = body {
        request_builder.json(&json_body)
    } else {
        request_builder
    };

    let request_builder = if let Some(query_params) = params {
        request_builder.query(&query_params)
    } else {
        request_builder
    };

    request_builder
        .send()
        .await
        .expect("Failed to execute request.")
}

pub async fn create_settings(pool: &SqlitePool, settings: &SettingsDBRow) -> Result<(), sqlx::Error> {
    let value_type_str = settings.value_type.to_string();

    sqlx::query(
        r#"
        INSERT INTO settings (key, value, type, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(&settings.key)
    .bind(&settings.value)
    .bind(&value_type_str)
    .bind(settings.created_at)
    .bind(settings.updated_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_settings(
    pool: &SqlitePool,
    key: &str,
) -> Result<Option<SettingsDBRow>, sqlx::Error> {
    let row = sqlx::query_as::<_, SettingsDBRow>(
        r#"
        SELECT key, value, type as value_type, created_at, updated_at
        FROM settings
        WHERE key = ?
        "#
    )
    .bind(key)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}
