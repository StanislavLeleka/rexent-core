use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LikedMovies {
    pub movies: Vec<i32>,
}
