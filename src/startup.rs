use std::net::TcpListener;

use actix_web::{
    body::MessageBody,
    dev::{Server, ServiceRequest, ServiceResponse},
    error,
    middleware::{from_fn, Next},
    web, App, Error, HttpResponse, HttpServer,
};
use actix_cors::Cors;

use crate::{
    errors::CustomError,
    models::{MessageResponse, SettingsDB},
    routes::{add_settings, get_settings, get_settings_by_key, health_check},
};

pub fn json_error_handler(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::JsonConfig::default().error_handler(|err, _req| {
        let err_message = err.to_string();

        // TODO: how to do it more elegantly? want to have only the error message without the line number
        let clean_error_message = match err_message.split(" at line").next() {
            Some(msg) => msg.to_string(),
            None => err_message,
        };

        CustomError::ValidationError(clean_error_message.to_string()).into()
    }));
}

async fn auth_middleware_new(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let headers = req.headers();
    let api_key_header = headers.get("X-Api-Key");

    if api_key_header.is_none() {
        // TODO: how to use custom error? `InternalError` looks redundant
        let error_body = MessageResponse {
            message: "missing `X-Api-Key` header".to_string(),
        };
        let error_response = HttpResponse::Unauthorized().json(error_body);
        return Err(error::InternalError::from_response("Unauthorized", error_response).into());
    }

    let expected_api_key = req.app_data::<String>().unwrap();
    if api_key_header.unwrap().to_str().unwrap() != expected_api_key {
        // TODO: how to use custom error? `InternalError` looks redundant
        let error_body = MessageResponse {
            message: "invalid `X-Api-Key` header".to_string(),
        };
        let error_response = HttpResponse::Forbidden().json(error_body);
        return Err(error::InternalError::from_response("Forbidden", error_response).into());
    }
    next.call(req).await
}

pub fn run(
    listener: TcpListener,
    db: SettingsDB,
    api_key: String,
) -> Result<Server, std::io::Error> {
    let db = web::Data::new(db);

    let server = HttpServer::new(move || {
        // TODO: now allow all, but in production should be more strict
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .service(
                web::scope("/api/v1/settings")
                    .wrap(from_fn(auth_middleware_new))
                    .route("", web::post().to(add_settings))
                    .route("", web::get().to(get_settings))
                    .route("/{key}", web::get().to(get_settings_by_key)),
            )
            .route("/health", web::get().to(health_check))
            .app_data(api_key.clone())
            .app_data(db.clone())
            .configure(json_error_handler)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
