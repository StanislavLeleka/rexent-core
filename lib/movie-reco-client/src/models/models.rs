use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PredictRequest {
    pub user_id: String,
    pub movie_ids: Vec<i32>,
    pub liked_movie_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictResponse {
    pub predictions: Vec<Prediction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prediction {
    pub movie_id: i32,
    pub tfidf: f32,
    pub cosine: f32,
}
