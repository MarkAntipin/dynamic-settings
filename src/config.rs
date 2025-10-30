use config::Environment;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_application_port")]
    pub application_port: u16,

    #[serde(default = "default_api_key")]
    pub api_key: String,

    #[serde(default = "default_sqlite_url")]
    pub sqlite_url: String,
}

impl Config {
    pub fn sqlite_filename(&self) -> String {
        // Extract filename from sqlite_url (e.g., "sqlite://dynamic-settings.db" -> "dynamic-settings.db")
        self.sqlite_url
            .strip_prefix("sqlite://")
            .unwrap_or(&self.sqlite_url)
            .to_string()
    }
}

fn default_application_port() -> u16 {
    18100
}

fn default_api_key() -> String {
    "api-key".to_string()
}

fn default_sqlite_url() -> String {
    "sqlite://dynamic-settings.db".to_string()
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    dotenv().ok();

    let mut conf = config::Config::default();

    conf.merge(Environment::default().separator(""))?;

    conf.try_into()
}
