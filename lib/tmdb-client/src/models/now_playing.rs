use serde::{Deserialize, Serialize};

use super::movie::Movie;

#[derive(Debug, Serialize, Deserialize)]
pub struct NowPlaying {
    pub dates: Date,
    pub page: i32,
    pub results: Vec<Movie>,
    pub total_pages: i32,
    pub total_results: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Date {
    pub maximum: String,
    pub minimum: String,
}
