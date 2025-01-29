use sqlx::PgPool;
use std::net::TcpListener;

use dynamic_settings::config::get_config;
use dynamic_settings::startup;

pub struct TestApp {
    pub address: String,
    pub pg_pool: PgPool,
    pub api_key: String,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let config = get_config().expect("Failed to read configuration.");
    let pg_pool = PgPool::connect(&config.pg_connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let server = startup::run(listener, pg_pool.clone(), config.api_key.clone())
        .expect("Failed to bind address");

    let _ = tokio::spawn(server);

    let test_app = TestApp {
        address: format!("http://127.0.0.1:{}", port),
        pg_pool: pg_pool,
        api_key: config.api_key,
    };
    test_app
}
