use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use tmdb_client::models::movie::Movie;

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryMovies {
    pub country: String,
    pub start_date: DateTime,
    pub end_date: DateTime,
    pub movies: Vec<Movie>,
}
