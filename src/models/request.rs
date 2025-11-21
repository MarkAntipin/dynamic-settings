use serde::{Deserialize, Serialize};

use crate::errors::CustomError;
use crate::enums::SettingsValueType;
use crate::utils::validate_settings_value;

const MAX_KEY_LENGTH: usize = 1_024;
const MAX_VALUE_LENGTH: usize = 4 * 1_024 * 1_024; /* 4 MiB */

#[derive(Serialize, Deserialize)]
pub struct CreateSettingsRequest {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: SettingsValueType,
}

impl CreateSettingsRequest {
    pub fn validate(&self) -> Result<(), CustomError> {
        if self.key.len() > MAX_KEY_LENGTH {
            return Err(CustomError::ValidationError(format!(
                "Key length should be less than {} bytes",
                MAX_KEY_LENGTH
            )));
        }

        if self.value.len() > MAX_VALUE_LENGTH {
            return Err(CustomError::ValidationError(format!(
                "Value length should be less than {} bytes",
                MAX_VALUE_LENGTH
            )));
        }
        validate_settings_value(self.value.clone(), self.value_type.clone())
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeleteSettingsByKeysRequest {
    pub keys: Vec<String>,
}

impl DeleteSettingsByKeysRequest {
    pub fn validate(&self) -> Result<(), CustomError> {
        if self.keys.is_empty() {
            return Err(CustomError::ValidationError(
                "Keys array should not be empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct ValidateTokenRequest {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateSettingsRequest {
    pub key: String,
    pub value: String,
}

impl UpdateSettingsRequest {
    pub fn validate(&self) -> Result<(), CustomError> {
        if self.value.len() > MAX_VALUE_LENGTH {
            return Err(CustomError::ValidationError(format!(
                "Value length should be less than {} bytes",
                MAX_VALUE_LENGTH
            )));
        }
        Ok(())
    }
}
