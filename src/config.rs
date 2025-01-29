use config::Environment;
use dotenv::dotenv;
use serde::Deserialize;

// # TODO: default values in Config
#[derive(Deserialize, Debug)]
pub struct Config {
    pub application_port: u16,
    pub api_key: String,

    pub pg_username: String,
    pub pg_password: String,
    pub pg_host: String,
    pub pg_port: u16,
    pub pg_database_name: String,
}

impl Config {
    pub fn pg_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.pg_username, self.pg_password, self.pg_host, self.pg_port, self.pg_database_name
        )
    }
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    dotenv().ok();

    let mut conf = config::Config::default();

    conf.merge(Environment::default().separator(""))?;

    conf.try_into()
}
