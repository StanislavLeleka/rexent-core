use actix_web::{web, HttpRequest, HttpResponse};
use auth::utils::get_user_id;
use proto::services::context::GetContextRequest;

use crate::{
    middleware::state::State,
    models::context::{Context, NewContextRequest},
};

pub async fn create_context(
    state: web::Data<State>,
    req: HttpRequest,
    new_context: web::Json<NewContextRequest>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let user_id = get_user_id(&req.headers())?;
    log::info!("Creating context for user: {}", user_id);

    let mut context_client = state.context_client.clone();

    let response = context_client
        .create_context(new_context.to_grpc(user_id.clone()))
        .await?;

    log::info!("Context created successfully for user: {}", user_id);
    Ok(HttpResponse::Ok().json(Context::from(response)))
}

pub async fn get_context(
    state: web::Data<State>,
    path: web::Path<String>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let user_id = path.to_string();
    log::info!("Fetching context for user: {}", user_id);

    let mut context_client = state.context_client.clone();

    let response = context_client
        .get_context(GetContextRequest {
            user_id: user_id.clone(),
        })
        .await?;

    log::info!("Context fetched successfully for user: {}", user_id);
    Ok(HttpResponse::Ok().json(Context::from(response)))
}
