use actix_web::web;
use actix_web::HttpResponse;

use crate::errors::CustomError;
use crate::models::{MessageResponse, SettingsDB, UpdateSettingsRequest};
use crate::repository::{db_update_settings_by_key};

pub async fn update_settings(
    db: web::Data<SettingsDB>,
    payload: web::Json<UpdateSettingsRequest>,
) -> Result<HttpResponse, CustomError> {
    let update_setting_payload = payload.into_inner();
    update_setting_payload.validate()?;

    let key = db_update_settings_by_key(
        &db, &update_setting_payload.key, &update_setting_payload.value
    )?;

    if key.is_none() {
        return Err(CustomError::NotFoundError(
            format!("Settings with key '{}' not found", update_setting_payload.key)
        ))
    }

    let response = MessageResponse {
        message: "Settings updated".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}
