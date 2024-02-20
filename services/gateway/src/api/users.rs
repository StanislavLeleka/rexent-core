use crate::models::users::NewSocialAccountRequest;
use crate::{
    middleware::state::State,
    models::users::{LoginRequest, NewUserRequest, TokenResponse, UserResponse},
};
use actix_web::{web, HttpRequest, HttpResponse};
use api_error::error::ApiError;
use auth::utils::get_user_id;

pub async fn sign_in(
    state: web::Data<State>,
    login_request: web::Json<LoginRequest>,
) -> Result<HttpResponse, ApiError> {
    log::info!("User attempting to sign in");

    let mut users_client = state.users_client.clone();
    let response = users_client.sign_in(login_request.0.into()).await?;

    log::info!("User signed in successfully");
    Ok(HttpResponse::Ok().json(TokenResponse::from(response)))
}

pub async fn sign_up(
    state: web::Data<State>,
    new_user_request: web::Json<NewUserRequest>,
) -> Result<HttpResponse, ApiError> {
    log::info!("User attempting to sign up");

    let mut users_client = state.users_client.clone();
    let response = users_client.sign_up(new_user_request.0.into()).await?;

    log::info!("User signed up successfully");
    Ok(HttpResponse::Ok().json(TokenResponse::from(response)))
}

pub async fn get_user(state: web::Data<State>, req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let user_id = get_user_id(req.headers())?;
    log::info!("Fetching details for user: {}", user_id);

    let mut users_client = state.users_client.clone();
    let response = users_client.get_user(user_id.clone()).await?;

    log::info!("User details fetched successfully for user: {}", user_id);
    Ok(HttpResponse::Ok().json(UserResponse::from(response)))
}

pub async fn link_social_account(
    state: web::Data<State>,
    req: HttpRequest,
    mut new_social_account: web::Json<NewSocialAccountRequest>,
) -> Result<HttpResponse, ApiError> {
    let user_id = get_user_id(req.headers())?;
    new_social_account.0.user_id = Some(user_id.clone());

    log::info!("User attempting to link social account: {}", user_id);

    let mut users_client = state.users_client.clone();
    users_client
        .link_social_account(new_social_account.0.into())
        .await?;

    log::info!("User linked social account successfully: {}", user_id);
    Ok(HttpResponse::Ok().json({}))
}
