use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Movie {
    pub id: i32,
    pub imdb_id: String,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub poster_path: String,
    pub release_date: String,
    pub title: String,
    pub vote_average: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MovieRecommendationsResponse {
    pub movies: Vec<Movie>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddUserRecommendationTypes {
    pub user_id: String,
    pub reco_type_ids: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRecommendationTypes {
    pub user_id: String,
    pub reco_types: Vec<RecommendationType>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecommendationType {
    pub reco_type_id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecommendationTypes {
    pub reco_types: Vec<RecommendationType>,
}

impl From<proto::services::movie::Movie> for Movie {
    fn from(value: proto::services::movie::Movie) -> Self {
        Movie {
            id: value.id,
            imdb_id: value.imdb_id,
            original_language: value.original_language,
            original_title: value.original_title,
            overview: value.overview,
            poster_path: value.poster_path,
            release_date: value.release_date,
            title: value.title,
            vote_average: value.vote_average,
        }
    }
}

impl From<proto::services::reco::RecoType> for RecommendationType {
    fn from(value: proto::services::reco::RecoType) -> Self {
        RecommendationType {
            reco_type_id: value.id,
            name: value.name,
            description: Some(value.description),
        }
    }
}
