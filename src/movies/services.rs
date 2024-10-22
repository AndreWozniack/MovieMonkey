use std::fmt::format;
use std::ptr::copy;
use env_logger::Logger;
use super::models::{Movie, MovieDetail, MovieError, MovieResponse};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use log::{debug, error, log_enabled, info, Level};


pub async fn fetch_popular_movies(api_key: &str) -> Result<MovieResponse, MovieError> {
    info!("Fetching popular movies");
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    let bearer_token = format!("Bearer {}", api_key);
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&bearer_token).unwrap());

    let url = "https://api.themoviedb.org/3/movie/popular";

    let response = client.get(url).headers(headers).send().await?;

    if response.status().is_success() {
        let movie_response: MovieResponse = response.json().await?;
        Ok(movie_response)
    } else {
        let status = response.status();
        let text = response.text().await?;
        eprintln!(
            "Erro ao obter filmes populares: Status: {}, Response: {}",
            status, text
        );

        Err(MovieError::StatusError(format!(
            "Status: {} - Response: {}",
            status, text
        )))
    }
}
pub async fn fetch_search_movies(api_key: &str, query: &str) -> Result<Vec<Movie>, reqwest::Error> {
    info!("Fetching movies with query: {}", query);
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    let bearer_token = format!("Bearer {}", api_key);
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&bearer_token).unwrap());

    let url = format!(
        "https://api.themoviedb.org/3/search/movie?query={}",
        query
    );
    let response = client.get(&url).headers(headers).send().await?;

    if response.status().is_success() {
        let movie_response: MovieResponse = response.json().await?;
        Ok(movie_response.results)
    } else {
        let status = response.status();
        let text = response.text().await?;
        eprintln!(
            "Erro ao obter filmes populares: Status: {}, Response: {}",
            status, text
        );

        Err(MovieError::StatusError(format!(
            "Status: {} - Response: {}",
            status, text
        ))).expect("TODO: panic message")

    }
}

pub async fn fetch_movie_detail(
    api_key: &str,
    movie_id: u32,
) -> Result<MovieDetail, reqwest::Error> {
    info!("Fetching movie detail for movie_id: {}", movie_id);
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    let bearer_token = format!("Bearer {}", api_key);
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&bearer_token).unwrap());

    let url = format!(
        "https://api.themoviedb.org/3/movie/{}",
        movie_id
    );

    let response = client.get(&url).headers(headers).send().await?;
    let movie_detail: MovieDetail = response.json().await?;
    Ok(movie_detail)
}
