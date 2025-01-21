use crate::errors::CustomError;
use crate::models::{MessageResponse, Settings};
use crate::repository::pg_add_settings;
use actix_web::web;
use actix_web::HttpResponse;
use sqlx::PgPool;

pub async fn add_settings(
    pool: web::Data<PgPool>,
    settings: web::Json<Settings>,
) -> Result<HttpResponse, CustomError> {
    let settings = settings.into_inner();
    settings.validate()?;

    let key = pg_add_settings(&pool, &settings).await?;

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
