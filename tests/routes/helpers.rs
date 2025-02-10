use dynamic_settings::config::get_config;
use dynamic_settings::models::{Settings, SettingsDB};
use dynamic_settings::startup;
use fjall::PartitionHandle;
use fjall::{Config, PartitionCreateOptions};
use std::net::TcpListener;
use tokio::sync::OnceCell;

pub struct TestApp {
    pub address: String,
    pub partition: PartitionHandle,
    pub api_key: String,
}

async fn setup_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let config = get_config().expect("Failed to read configuration.");

    let keyspace = Config::new("db")
        .open()
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

pub fn add_settings(partition: &PartitionHandle, settings: &Settings) {
    let key = &settings.key;
    let serialized: Vec<u8> = settings.into();
    partition
        .insert(key, serialized)
        .expect("Failed to insert settings");
}

pub fn get_settings(
    partition: &PartitionHandle,
    key: &str,
) -> Result<Option<Settings>, fjall::Error> {
    let Some(item) = partition.get(key)? else {
        return Ok(None);
    };

    let settings: Settings =
        rmp_serde::from_slice(&item).expect("Error deserializing settings from bytes");
    Ok(Some(settings))
}
