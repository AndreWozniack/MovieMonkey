pub struct ApiState {
    pub tmdb_api_key: String,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub genre_ids: Vec<u32>,
    pub id: u32,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub release_date: Option<String>,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MovieDetail {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub belongs_to_collection: Option<String>, // Pode ser nulo
    pub budget: u64,
    pub genres: Vec<Genre>,       // Lista de objetos de gênero
    pub homepage: Option<String>, // Pode ser nulo
    pub id: u32,
    pub imdb_id: Option<String>, // Pode ser nulo
    pub original_language: String,
    pub original_title: String,
    pub overview: Option<String>, // Pode ser nulo
    pub popularity: f64,
    pub poster_path: Option<String>,                  // Pode ser nulo
    pub production_companies: Vec<ProductionCompany>, // Lista de objetos de empresas de produção
    pub production_countries: Vec<ProductionCountry>, // Lista de objetos de países de produção
    pub release_date: Option<String>,                 // Pode ser nulo
    pub revenue: u64,
    pub runtime: Option<u32>,                  // Pode ser nulo
    pub spoken_languages: Vec<SpokenLanguage>, // Lista de objetos de idiomas falados
    pub status: String,
    pub tagline: Option<String>, // Pode ser nulo
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Genre {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductionCompany {
    pub id: u32,
    pub logo_path: Option<String>, // Pode ser nulo
    pub name: String,
    pub origin_country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductionCountry {
    pub iso_3166_1: String, // Código ISO do país
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpokenLanguage {
    pub english_name: String,
    pub iso_639_1: String,
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct MovieResponse {
    pub page: u32,
    pub results: Vec<Movie>,
    pub total_pages: u32,
    pub total_results: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MovieError {
    #[error("Erro de requisição: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Erro de status: {0}")]
    StatusError(String),
}
