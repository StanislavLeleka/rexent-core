use proto::services::reco::{
    GetMovieRecommendationsResponse, GetRecommendationTypesResponse,
    GetUserRecommendationTypesResponse,
};
use tmdb_client::models::movie::Movie;

use super::user::{ExtendedUserRecommendation, RecommendationType};

pub struct Mapper;

impl Mapper {
    pub fn map_to_movie_reco_response(
        recommendations: Vec<Movie>,
    ) -> GetMovieRecommendationsResponse {
        let mut response = GetMovieRecommendationsResponse::default();
        response.movies = recommendations
            .into_iter()
            .map(|m| proto::services::movie::Movie {
                id: m.id,
                imdb_id: "".to_string(),
                original_language: m.original_language.unwrap(),
                original_title: m.original_title.unwrap(),
                overview: m.overview.unwrap(),
                poster_path: m.poster_path.unwrap(),
                release_date: m.release_date.unwrap(),
                title: m.title.unwrap(),
                vote_average: m.vote_average,
                genres: vec![],
            })
            .collect();
        response
    }

    pub fn map_to_user_reco_response(
        recommendations: Vec<ExtendedUserRecommendation>,
    ) -> GetUserRecommendationTypesResponse {
        let mut response = GetUserRecommendationTypesResponse::default();
        response.reco_types = recommendations
            .into_iter()
            .map(|r| proto::services::reco::RecoType {
                id: r.reco_id,
                name: r.name,
                description: r.description.unwrap_or_default(),
            })
            .collect();
        response
    }

    pub fn map_to_reco_type_response(
        reco_types: Vec<RecommendationType>,
    ) -> GetRecommendationTypesResponse {
        let mut response = GetRecommendationTypesResponse::default();
        response.reco_types = reco_types
            .into_iter()
            .map(|r| proto::services::reco::RecoType {
                id: r.reco_type_id,
                name: r.name,
                description: r.description.unwrap_or_default(),
            })
            .collect();
        response
    }
}
