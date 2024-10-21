use dotenv::dotenv;
use movie_monkey::run;
use std::env;
use std::net::TcpListener;

mod movies;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let api_key = env::var("TMDB_API_KEY").expect("TMDB_API_KEY must be set");
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address)?;
    println!("Server is running on http://{}", address);
    run(listener, api_key)?.await
}
