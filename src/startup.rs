use crate::routes::add_settings;
use crate::routes::get_settings;
use crate::routes::get_settings_by_key;
use crate::routes::health_check;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::errors::CustomError;

pub fn json_error_handler(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::JsonConfig::default().error_handler(|err, _req| {
        let err_message = err.to_string();

        // TODO: how to do it more elegantly? want to have only the error message without the line number
        let clean_message = match err_message.split(" at line").next() {
            Some(msg) => msg.to_string(),
            None => err_message,
        };

        CustomError::ValidationError(clean_message.to_string()).into()
    }));
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/api/v1/settings", web::post().to(add_settings))
            .route("/api/v1/settings/{key}", web::get().to(get_settings_by_key))
            .route("/api/v1/settings", web::get().to(get_settings))
            .app_data(db_pool.clone())
            .configure(json_error_handler)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
