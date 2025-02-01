use actix_web::web;
use actix_web::HttpResponse;

use crate::errors::CustomError;
use crate::models::{MessageResponse, SettingsDB};
use crate::repository::{db_get_settings, db_get_settings_by_key};

pub async fn get_settings_by_key(
    db: web::Data<SettingsDB>,
    key: web::Path<String>,
) -> Result<HttpResponse, CustomError> {
    let settings = db_get_settings_by_key(&db, &key)?;
    if settings.is_none() {
        // TODO: does it better to use custom error?
        // return Err(CustomError::NotFoundError(format!(
        //     "Settings with key '{}' not found",
        //     key
        // )));
        let response = MessageResponse {
            message: format!("Settings with key '{}' not found", key),
        };
        return Ok(HttpResponse::NotFound().json(response));
    }
    Ok(HttpResponse::Ok().json(settings))
}

pub async fn get_settings(db: web::Data<SettingsDB>) -> Result<HttpResponse, CustomError> {
    let settings = db_get_settings(&db)?;
    Ok(HttpResponse::Ok().json(settings))
}
