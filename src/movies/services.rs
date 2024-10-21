use super::models::{Movie, MovieDetail, MovieError, MovieResponse};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

pub async fn fetch_popular_movies(api_key: &str) -> Result<MovieResponse, MovieError> {
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
    let url = format!(
        "https://api.themoviedb.org/3/search/movie?api_key={}&query={}",
        api_key, query
    );
    let response = reqwest::get(&url).await?;
    let movies: MovieResponse = response.json().await?;
    Ok(movies.results)
}

pub async fn fetch_movie_detail(
    api_key: &str,
    movie_id: u32,
) -> Result<MovieDetail, reqwest::Error> {
    let url = format!(
        "https://api.themoviedb.org/3/movie/{}?api_key={}",
        movie_id, api_key
    );
    let response = reqwest::get(&url).await?;
    let movie_detail: MovieDetail = response.json().await?;
    Ok(movie_detail)
}
