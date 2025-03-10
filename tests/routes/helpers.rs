use std::option::Option;

use dynamic_settings::config::get_config;
use dynamic_settings::models::{SettingsDBRow, SettingsDB};
use dynamic_settings::startup;
use fjall::TxPartitionHandle;
use fjall::{Config, PartitionCreateOptions};
use std::net::TcpListener;
use reqwest::header::{HeaderMap, HeaderValue};
use tokio::sync::OnceCell;

pub struct TestApp {
    pub address: String,
    pub partition: TxPartitionHandle,
    pub api_key: String,
}

async fn setup_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let config = get_config().expect("Failed to read configuration.");

    let keyspace = Config::new("db")
        .open_transactional()
        .expect("Failed connect to keyspace");
    let partition = keyspace
        .open_partition("settings", PartitionCreateOptions::default())
        .expect("Failed to connect to partition");
    let settings_db = SettingsDB {
        keyspace,
        partition: partition.clone(),
    };

    let server = startup::run(listener, settings_db, config.api_key.clone())
        .expect("Failed to bind address");

    tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        partition,
        api_key: config.api_key,
    }
}

static INIT: OnceCell<TestApp> = OnceCell::const_new();

// setup_app called only once
pub async fn spawn_app() -> &'static TestApp {
    INIT.get_or_init(setup_app).await
}


pub async fn make_request(
    url: String,
    api_key: String,
    body: Option<serde_json::Value>,
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

    request_builder
        .send()
        .await
        .expect("Failed to execute request.")
}


pub fn create_settings(partition: &TxPartitionHandle, settings: &SettingsDBRow) {
    let key = &settings.key;
    let serialized: Vec<u8> = settings.into();
    partition
        .insert(key, serialized)
        .expect("Failed to insert settings");
}

pub fn get_settings(
    partition: &TxPartitionHandle,
    key: &str,
) -> Result<Option<SettingsDBRow>, fjall::Error> {
    let Some(item) = partition.get(key)? else {
        return Ok(None);
    };

    let settings: SettingsDBRow =
        rmp_serde::from_slice(&item).expect("Error deserializing settings from bytes");
    Ok(Some(settings))
}
