use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_session::{SessionMiddleware, storage::RedisActorSessionStore};
use actix_web::cookie::Key;
use serde::{Serialize};
use actix_cors::Cors;

mod api;
mod models;
mod repository;
mod utils;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let usr_db = repository::database::Database::new();
    let app_data = web::Data::new(usr_db);
    let secret_key = Key::generate();
    let redis_connection_string = "127.0.0.1:6379";
    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .configure(api::controller::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(Cors::permissive())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(
                SessionMiddleware::new(
                    RedisActorSessionStore::new(redis_connection_string),
                    secret_key.clone()
                )
            )
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}