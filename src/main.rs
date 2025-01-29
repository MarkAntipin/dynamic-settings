use dynamic_settings::config::get_config;
use dynamic_settings::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read configuration.");

    let connection_pool = PgPool::connect(&config.pg_connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool, config.api_key)?.await
}
