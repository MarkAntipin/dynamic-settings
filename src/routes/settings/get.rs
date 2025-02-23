use std::cmp::Reverse;

use actix_web::web;
use actix_web::HttpResponse;

use crate::errors::CustomError;
use crate::models::SettingsDB;
use crate::repository::{db_get_settings, db_get_settings_by_key};

pub async fn get_settings_by_key(
    db: web::Data<SettingsDB>,
    key: web::Path<String>,
) -> Result<HttpResponse, CustomError> {
    let settings_row = db_get_settings_by_key(&db, &key)?;
    if settings_row.is_none() {
        return Err(CustomError::NotFoundError(format!("Settings with key '{}' not found",  key)));
    }
    Ok(HttpResponse::Ok().json(settings_row))
}

pub async fn get_settings(db: web::Data<SettingsDB>) -> Result<HttpResponse, CustomError> {
    let mut settings_rows = db_get_settings(&db)?;
    settings_rows.sort_by_key(|s| Reverse(s.created_at));
    Ok(HttpResponse::Ok().json(settings_rows))
}
