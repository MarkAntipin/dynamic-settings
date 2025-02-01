use crate::errors::CustomError;
use crate::models::{MessageResponse, Settings, SettingsDB};
use crate::repository::db_add_settings;
use actix_web::web;
use actix_web::HttpResponse;

pub async fn add_settings(
    db: web::Data<SettingsDB>,
    settings: web::Json<Settings>,
) -> Result<HttpResponse, CustomError> {
    let settings = settings.into_inner();
    settings.validate()?;

    let key = db_add_settings(&db, &settings)?;

    if key.is_none() {
        let response = MessageResponse {
            message: format!("Settings with key '{}' already exist", settings.key),
        };
        Ok(HttpResponse::Conflict().json(response))
    } else {
        let response = MessageResponse {
            message: format!("Settings with key '{}' created", settings.key),
        };
        Ok(HttpResponse::Created().json(response))
    }
}
