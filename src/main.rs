use std::net::TcpListener;
use env_logger::Env;
use dynamic_settings::config::get_config;
use dynamic_settings::models::SettingsDB;
use dynamic_settings::startup::run;
use sqlx::migrate::Migrator;
use sqlx::sqlite::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read configuration.");

    let db_options = SqliteConnectOptions::new()
        .filename(config.sqlite_filename())
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(db_options)
        .await
        .expect("Failed to create database pool");

    MIGRATOR.run(&pool).await.expect("Migration failed");

    let settings_db = SettingsDB {
        pool
    };

    let address = format!("0.0.0.0:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    run(listener, settings_db, config.api_key)?.await
}
