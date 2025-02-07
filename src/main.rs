use std::net::TcpListener;

use fjall::{Config, PartitionCreateOptions};

use dynamic_settings::config::get_config;
use dynamic_settings::models::SettingsDB;
use dynamic_settings::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read configuration.");

    let keyspace = Config::new("db")
        .open()
        .expect("Failed connect to keyspace");
    let partition = keyspace
        .open_partition("settings", PartitionCreateOptions::default())
        .expect("Failed to connect to partition");

    let settings_db = SettingsDB {
        keyspace,
        partition,
    };

    let address = format!("0.0.0.0:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, settings_db, config.api_key)?.await
}
