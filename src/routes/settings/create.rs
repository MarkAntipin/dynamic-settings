use actix_web::web;
use actix_web::HttpResponse;

use crate::errors::CustomError;
use crate::models::{MessageResponse, SettingsRequest, SettingsDBRow, SettingsDB};
use crate::repository::db_create_settings;

pub async fn create_settings(
    db: web::Data<SettingsDB>,
    payload: web::Json<SettingsRequest>,
) -> Result<HttpResponse, CustomError> {
    let settings = payload.into_inner();
    settings.validate()?;

    let settings_row: SettingsDBRow = settings.into();
    let key = db_create_settings(&db, &settings_row)?;

    if key.is_none() {
        return Err(CustomError::ConflictError(
            format!("Settings with key '{}' already exist", settings_row.key)
        ))
    }
    let response = MessageResponse {
        message: format!("Settings with key '{}' created", settings_row.key),
    };
    Ok(HttpResponse::Created().json(response))
}
