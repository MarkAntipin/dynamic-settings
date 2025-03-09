use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetSettingsQueryParams {
    #[serde(default)]
    pub prefix: String,
}
