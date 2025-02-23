use config::Environment;
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_application_port")]
    pub application_port: u16,

    #[serde(default = "default_api_key")]
    pub api_key: String,
}

fn default_application_port() -> u16 {
    18100
}

fn default_api_key() -> String {
    "api-key".to_string()
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    dotenv().ok();

    let mut conf = config::Config::default();

    conf.merge(Environment::default().separator(""))?;

    conf.try_into()
}
