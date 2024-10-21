use super::models::{ApiState, SearchQuery};
use super::services;
use actix_web::web::Data;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/popular")]
pub async fn get_popular_movies(data: Data<ApiState>) -> impl Responder {
    match services::fetch_popular_movies(&data.tmdb_api_key).await {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(err) => {
            eprintln!("Erro ao obter filmes populares: {:?}", err);
            HttpResponse::InternalServerError().body("Erro ao obter filmes populares")
        }
    }
}

#[get("/search")]
pub async fn search_movies(data: Data<ApiState>, query: web::Query<SearchQuery>) -> impl Responder {
    match services::fetch_search_movies(&data.tmdb_api_key, &query.query).await {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(err) => {
            eprintln!("Erro ao buscar filmes: {:?}", err);
            HttpResponse::InternalServerError().body("Erro ao buscar filmes")
        }
    }
}

#[get("/{id}")]
pub async fn get_movie_detail(data: web::Data<ApiState>, path: web::Path<u32>) -> impl Responder {
    let movie_id = path.into_inner();
    match services::fetch_movie_detail(&data.tmdb_api_key, movie_id).await {
        Ok(movie_detail) => HttpResponse::Ok().json(movie_detail),
        Err(err) => {
            eprintln!("Erro ao obter detalhes do filme: {:?}", err);
            HttpResponse::InternalServerError().body("Erro ao obter detalhes do filme")
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_popular_movies)
        .service(search_movies)
        .service(get_movie_detail)
        .service(get_popular_movies);
}