use crate::{models::threshold::Threshold, storage::movie::MovieStorage};
use movie_reco_client::{
    models::models::{PredictRequest, PredictResponse},
    movie_reco_client::MovieRecoClient,
};
use proto::{
    clients::{context::ContextServiceGrpcClient, favorites::FavoritesServiceGrpcClient},
    services::{
        context::{ContextResponse, GetContextRequest},
        favorites::{GetLikedMoviesRequest, GetLikedMoviesResponse},
    },
};
use std::collections::HashMap;
use tmdb_client::models::movie::Movie;
use tonic::Status;

pub struct MovieRecommender {
    pub movie_reco_client: MovieRecoClient,
    pub favorites_client: FavoritesServiceGrpcClient,
    pub context_client: ContextServiceGrpcClient,
    pub movie_storage: MovieStorage,
    pub threshold: Threshold,
}

impl MovieRecommender {
    pub fn new(
        movie_reco_client: MovieRecoClient,
        favorites_client: FavoritesServiceGrpcClient,
        context_client: ContextServiceGrpcClient,
        movie_storage: MovieStorage,
    ) -> Self {
        Self {
            movie_reco_client,
            favorites_client,
            context_client,
            movie_storage,
            threshold: Threshold::from_env(),
        }
    }

    pub async fn get_recommendations(&self, user_id: String) -> Result<Vec<Movie>, Status> {
        let context = self.get_context(user_id.clone()).await?;
        let liked_movies = self.get_liked_movies(user_id.clone()).await?;
        let country_code = self.get_country_code(&context)?;
        let country_movies = self.get_country_movies(country_code).await?;
        let movies_map = self.create_movies_map(country_movies);

        let prediction_result = self
            .get_prediction_result(user_id.clone(), &movies_map, liked_movies)
            .await?;
        let recommendations = self.get_filtered_recommendations(prediction_result, &movies_map);

        Ok(recommendations)
    }

    async fn get_context(&self, user_id: String) -> Result<ContextResponse, Status> {
        let mut context_client = self.context_client.clone();
        let context = context_client
            .get_context(GetContextRequest { user_id })
            .await?;

        Ok(context)
    }

    async fn get_liked_movies(&self, user_id: String) -> Result<GetLikedMoviesResponse, Status> {
        let mut favorites_client = self.favorites_client.clone();
        let liked_movies = favorites_client
            .get_liked_movies(GetLikedMoviesRequest { user_id })
            .await?;

        Ok(liked_movies)
    }

    fn get_country_code(&self, context: &ContextResponse) -> Result<String, Status> {
        match context.context.as_ref().and_then(|c| c.location.as_ref()) {
            Some(location) => Ok(location.country_code().to_string()),
            None => {
                log::error!("Failed to get country code from context");
                Err(Status::internal("Failed to get country code from context"))
            }
        }
    }

    async fn get_country_movies(&self, country_code: String) -> Result<Vec<Movie>, Status> {
        let country_movies = self
            .movie_storage
            .get_country_movies(country_code.clone())
            .await
            .map_err(|err| {
                log::error!("Error getting country movies: {:?}", err);
                Status::internal("Failed to get country movies")
            })?
            .ok_or_else(|| {
                log::error!("No country movies found for country: {}", country_code);
                Status::not_found("Country movies not found")
            })?;

        Ok(country_movies.movies)
    }

    fn create_movies_map(&self, movies: Vec<Movie>) -> HashMap<i32, Movie> {
        movies.into_iter().map(|m| (m.id, m)).collect()
    }

    async fn get_prediction_result(
        &self,
        user_id: String,
        movies_map: &HashMap<i32, Movie>,
        liked_movies: GetLikedMoviesResponse,
    ) -> Result<PredictResponse, Status> {
        let prediction_result = self
            .movie_reco_client
            .predict(PredictRequest {
                user_id,
                movie_ids: movies_map.keys().cloned().collect(),
                liked_movie_ids: liked_movies
                    .movie_ids
                    .into_iter()
                    .map(|m| m)
                    .collect::<Vec<i32>>(),
            })
            .await
            .map_err(|err| {
                log::info!("Error getting movie recommendations: {:?}", err);
                Status::internal("Failed to get movie recommendations")
            })?;

        Ok(prediction_result)
    }

    fn get_filtered_recommendations(
        &self,
        prediction_result: PredictResponse,
        movies_map: &HashMap<i32, Movie>,
    ) -> Vec<Movie> {
        prediction_result
            .predictions
            .iter()
            .filter_map(|prediction| {
                if self.get_weighted_prediction(prediction.tfidf, prediction.cosine)
                    >= self.threshold.accuracy
                {
                    movies_map.get(&prediction.movie_id).cloned()
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_weighted_prediction(&self, tfidf: f32, cosine: f32) -> f32 {
        ((tfidf / self.threshold.tfidf) + (cosine / self.threshold.cosine)) / 2.0
    }
}
