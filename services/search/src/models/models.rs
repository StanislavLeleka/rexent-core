use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub(crate) id: u32,
    pub(crate) imdb_id: String,
    pub(crate) original_language: String,
    pub(crate) original_title: String,
    pub(crate) overview: String,
    pub(crate) poster_path: String,
    pub(crate) release_date: String,
    pub(crate) title: String,
    pub(crate) vote_average: f32,
    pub(crate) genres: Vec<Genre>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Genre {
    pub(crate) id: u32,
    pub(crate) name: String,
}
