mod health;
mod movies;

use crate::movies::models::ApiState;
use crate::movies::handlers::init_routes;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener, api_key: String) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(ApiState {
                tmdb_api_key: api_key.clone(),
            }))
            .configure(health::init_routes)
            .service(web::scope("/movies").configure(init_routes))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
