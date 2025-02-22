use actix_web::web;
use actix_web::HttpResponse;

use crate::errors::CustomError;
use crate::models::{MessageResponse, SettingsDB, DeleteSettingsByKeysRequest};
use crate::repository::db_delete_settings_by_keys;

pub async fn delete_settings(
    db: web::Data<SettingsDB>,
    payload: web::Json<DeleteSettingsByKeysRequest>,
) -> Result<HttpResponse, CustomError> {
    let setting_keys = payload.into_inner();

    db_delete_settings_by_keys(&db, setting_keys.keys)?;
    let response = MessageResponse {
        message: "Settings deleted".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}
