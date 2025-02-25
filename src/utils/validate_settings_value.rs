use crate::enums::SettingsValueType;
use crate::errors::CustomError;

pub fn validate_settings_value(
    value: String,
    value_type: SettingsValueType,
) -> Result<(), CustomError>  {
    match value_type {
        SettingsValueType::Str => Ok(()),
        SettingsValueType::Int => {
            value.parse::<i64>().map_err(|_| {
                CustomError::ValidationError(format!(
                    "Value '{}' is not a valid integer",
                    value
                ))
            })?;
            Ok(())
        }
        SettingsValueType::Bool => {
            value.parse::<bool>().map_err(|_| {
                CustomError::ValidationError(format!(
                    "Value '{}' is not a valid boolean",
                    value
                ))
            })?;
            Ok(())
        }
        SettingsValueType::Float => {
            value.parse::<f64>().map_err(|_| {
                CustomError::ValidationError(format!(
                    "Value '{}' is not a valid float",
                    value
                ))
            })?;
            Ok(())
        }
        SettingsValueType::Json => {
            serde_json::from_str::<serde_json::Value>(&value).map_err(|_| {
                CustomError::ValidationError(format!(
                    "Value '{}' is not a valid JSON",
                    value
                ))
            })?;
            Ok(())
        }
    }
}
