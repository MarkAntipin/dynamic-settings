use std::net::TcpListener;

use actix_web::{
    body::MessageBody,
    dev::{Server, ServiceRequest, ServiceResponse},
    error,
    middleware::{from_fn, Next, Logger},
    web, App, Error, HttpResponse, HttpServer
};
use actix_cors::Cors;
use actix_files;
use env_logger::Env;

use crate::{
    errors::CustomError,
    models::{MessageResponse, SettingsDB},
    routes::{add_settings, get_settings, get_settings_by_key, delete_settings, health_check, validate_token},
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

    let expected_api_key = req.app_data::<web::Data<String>>().unwrap();
    if api_key_header.unwrap().to_str().unwrap() != expected_api_key.to_string() {
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
    let api_key = web::Data::new(api_key);
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let server = HttpServer::new(move || {
        // TODO: now allow all, but in production should be more strict
        let cors = Cors::permissive();

        let mut app = App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                .service(
                    web::scope("/settings")
                        .wrap(from_fn(auth_middleware_new))
                        .route("", web::post().to(add_settings))
                        .route("", web::get().to(get_settings))
                        .route("", web::delete().to(delete_settings))
                        .route("/{key}", web::get().to(get_settings_by_key))
                )
                .service(
                    web::scope("/auth")
                        .route("/validate-token", web::post().to(validate_token))
                )
            )
            .route("/health", web::get().to(health_check))
            .app_data(api_key.clone())
            .app_data(db.clone())
            .configure(json_error_handler);

        // if release build, serve the UI
        if !cfg!(debug_assertions) {
            app = app.service(actix_files::Files::new("/", "./ui/dist").index_file("index.html"));
        }
        app
    })
    .listen(listener)?
    .run();
    Ok(server)
}
