use crate::{
    middleware::state::State,
    models::location::{CountriesResponse, LocationResponse, NewLocation, UpdateLocation},
};
use actix_web::{web, HttpRequest, HttpResponse};
use api_error::error::ApiError;
use auth::utils::get_user_id;
use proto::services::location::{GetCountriesWithCitiesRequest, GetLocationRequest};
use serde_json::json;

pub async fn add_location(
    state: web::Data<State>,
    req: HttpRequest,
    new_location: web::Json<NewLocation>,
) -> Result<HttpResponse, ApiError> {
    let user_id = get_user_id(req.headers())?;
    log::info!("Adding location for user: {}", user_id);

    let mut location_client = state.loc_client.clone();

    let response = location_client
        .add_location(new_location.to_grpc(user_id.clone()))
        .await?;

    log::info!("Location added successfully for user: {}", user_id);
    Ok(HttpResponse::Ok().json(LocationResponse::from(response)))
}

pub async fn update_location(
    state: web::Data<State>,
    req: HttpRequest,
    update_location: web::Json<UpdateLocation>,
) -> Result<HttpResponse, ApiError> {
    let user_id = get_user_id(req.headers())?;
    log::info!("Updating location for user: {}", user_id);

    let mut location_client = state.loc_client.clone();

    let response = location_client
        .update_location(update_location.to_grpc(user_id.clone()))
        .await?;

    log::info!("Location updated successfully for user: {}", user_id);
    Ok(HttpResponse::Ok().json(LocationResponse::from(response)))
}

pub async fn get_location(
    state: web::Data<State>,
    req: HttpRequest,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let user_id = get_user_id(req.headers())?;
    log::info!("Fetching location for user: {}", user_id);

    check_user_id(&path, &user_id)?;

    let mut location_client = state.loc_client.clone();

    let response = location_client
        .get_location(GetLocationRequest {
            user_id: user_id.clone(),
        })
        .await?;

    log::info!("Location fetched successfully for user: {}", user_id);
    Ok(HttpResponse::Ok().json(LocationResponse::from(response)))
}

pub async fn get_countries_with_cities(state: web::Data<State>) -> Result<HttpResponse, ApiError> {
    log::info!("Fetching countries with cities");

    let mut location_client = state.loc_client.clone();

    let response = location_client
        .get_countries_with_cities(GetCountriesWithCitiesRequest {})
        .await?;

    log::info!("Countries with cities fetched successfully");
    Ok(HttpResponse::Ok().json(CountriesResponse::from(response)))
}

fn check_user_id(path: &str, user_id: &str) -> Result<(), ApiError> {
    if path != user_id {
        log::error!("Invalid token for user ID: {}", user_id);
        return Err(ApiError::Unauthorized(json!({"error": "Invalid token"})));
    }
    Ok(())
}
