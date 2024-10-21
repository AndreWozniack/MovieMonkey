use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MessageResponse {
    message: String,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn get_message() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(health_check))
        .route("/health", web::get().to(health_check))
        .route("/message", web::get().to(get_message));
}
