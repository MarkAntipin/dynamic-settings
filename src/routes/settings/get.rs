use actix_web::web;
use actix_web::HttpResponse;
use sqlx::PgPool;

use crate::errors::CustomError;
use crate::models::MessageResponse;
use crate::repository::{pg_get_settings, pg_get_settings_by_key};

pub async fn get_settings_by_key(
    pool: web::Data<PgPool>,
    key: web::Path<String>,
) -> Result<HttpResponse, CustomError> {
    let settings = pg_get_settings_by_key(&pool, &key).await?;
    if settings.is_none() {
        let response = MessageResponse {
            message: format!("Settings with key '{}' not found", key),
        };
        return Ok(HttpResponse::NotFound().json(response));
    }
    Ok(HttpResponse::Ok().json(settings))
}

pub async fn get_settings(pool: web::Data<PgPool>) -> Result<HttpResponse, CustomError> {
    let settings = pg_get_settings(&pool).await?;
    Ok(HttpResponse::Ok().json(settings))
}
