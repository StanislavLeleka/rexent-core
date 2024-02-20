use crate::middleware::state::State;
use crate::models::events::{GetShowtimesRequest, Showtimes};
use actix_web::{web, HttpRequest, HttpResponse};
use api_error::error::ApiError;
use auth::utils::get_user_id;

pub async fn get_showtimes(
    state: web::Data<State>,
    req: HttpRequest,
    get_showtimes_request: web::Json<GetShowtimesRequest>,
) -> Result<HttpResponse, ApiError> {
    let user_id = get_user_id(req.headers())?;
    log::info!("Fetching showtimes for user: {}", user_id);

    let mut events_client = state.events_client.clone();
    let response = events_client
        .get_showtimes(get_showtimes_request.0.into())
        .await?;

    log::info!("Showtimes fetched successfully for user: {}", user_id);
    Ok(HttpResponse::Ok().json(Showtimes::from(response)))
}
