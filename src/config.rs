use config::Environment;
use dotenv::dotenv;
use serde::Deserialize;

// # TODO: default values in Config
#[derive(Deserialize, Debug)]
pub struct Config {
    pub application_port: u16,
    pub api_key: String,
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    dotenv().ok();

    let mut conf = config::Config::default();

    conf.merge(Environment::default().separator(""))?;

    conf.try_into()
}
