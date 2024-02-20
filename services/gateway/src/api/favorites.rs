use crate::{middleware::state::State, models::favorites::LikedMovies};
use actix_web::{web, HttpRequest, HttpResponse};
use api_error::error::ApiError;
use auth::utils::get_user_id;
use proto::services::favorites::{AddLikedMoviesRequest, GetLikedMoviesRequest};

pub async fn add_liked_movies(
    state: web::Data<State>,
    req: HttpRequest,
    liked_movies: web::Json<LikedMovies>,
) -> Result<HttpResponse, ApiError> {
    let user_id = get_user_id(req.headers())?;
    log::info!("Adding liked movies for user: {}", user_id);

    let mut favorites_client = state.favorites_client.clone();

    favorites_client
        .add_liked_movies(AddLikedMoviesRequest {
            user_id: user_id.clone(),
            movie_id: liked_movies.movies.clone(),
        })
        .await?;

    log::info!("Liked movies added successfully for user: {}", user_id);
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_liked_movies(
    state: web::Data<State>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.to_string();
    log::info!("Fetching liked movies for user: {}", user_id);

    let mut favorites_client = state.favorites_client.clone();

    let response = favorites_client
        .get_liked_movies(GetLikedMoviesRequest {
            user_id: user_id.clone(),
        })
        .await?;

    log::info!("Liked movies fetched successfully for user: {}", user_id);
    Ok(HttpResponse::Ok().json(LikedMovies {
        movies: response.movie_ids,
    }))
}
