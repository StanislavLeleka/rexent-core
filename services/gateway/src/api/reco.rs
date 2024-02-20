use crate::{
    middleware::state::State,
    models::reco::{
        AddUserRecommendationTypes, MovieRecommendationsResponse, RecommendationTypes,
        UserRecommendationTypes,
    },
};
use actix_web::{web, HttpResponse};
use proto::services::reco::{
    AddUserRecommendationTypesRequest, GetMovieRecommendationsRequest,
    GetRecommendationTypesRequest, GetUserRecommendationTypesRequest,
};

pub async fn get_movie_recommendations(
    state: web::Data<State>,
    path: web::Path<String>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let user_id = path.to_string();
    log::info!("Fetching movie recommendations for user: {}", user_id);

    let mut reco_client = state.reco_client.clone();
    let response = reco_client
        .get_movie_recommendations(GetMovieRecommendationsRequest {
            user_id: user_id.clone(),
            max_recommendations: 10,
        })
        .await?;

    log::info!(
        "Movie recommendations fetched successfully for user: {}",
        user_id
    );

    Ok(HttpResponse::Ok().json(MovieRecommendationsResponse {
        movies: response.movies.into_iter().map(|m| m.into()).collect(),
    }))
}

pub async fn add_user_recommendation_types(
    state: web::Data<State>,
    path: web::Path<String>,
    reco_types: web::Json<AddUserRecommendationTypes>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let user_id = path.to_string();
    log::info!("Adding recommendation types for user: {}", user_id);

    let mut reco_client = state.reco_client.clone();
    let response = reco_client
        .add_user_recommendation_types(AddUserRecommendationTypesRequest {
            user_id: user_id.clone(),
            reco_ids: reco_types.reco_type_ids.clone(),
        })
        .await?;

    log::info!(
        "Recommendation types successfully added for user: {}",
        user_id
    );

    Ok(HttpResponse::Ok().finish())
}

pub async fn get_user_recommendation_types(
    state: web::Data<State>,
    path: web::Path<String>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let user_id = path.to_string();
    log::info!("Fetching recommendation types for user: {}", user_id);

    let mut reco_client = state.reco_client.clone();
    let response = reco_client
        .get_user_recommendation_types(GetUserRecommendationTypesRequest {
            user_id: user_id.clone(),
        })
        .await?;

    log::info!(
        "Recommendation types fetched successfully for user: {}",
        user_id
    );

    Ok(HttpResponse::Ok().json(UserRecommendationTypes {
        user_id,
        reco_types: response.reco_types.into_iter().map(|r| r.into()).collect(),
    }))
}

pub async fn get_recommendation_types(
    state: web::Data<State>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    log::info!("Fetching all recommendation types");

    let mut reco_client = state.reco_client.clone();
    let response = reco_client
        .get_recommendation_types(GetRecommendationTypesRequest {})
        .await?;

    log::info!("Recommendation types fetched successfully");

    Ok(HttpResponse::Ok().json(RecommendationTypes {
        reco_types: response.reco_types.into_iter().map(|r| r.into()).collect(),
    }))
}
