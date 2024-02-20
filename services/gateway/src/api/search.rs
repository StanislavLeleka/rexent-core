use crate::{
    middleware::state::State,
    models::search::{SearchQuery, SearchResult},
};
use actix_web::{web, HttpResponse};
use api_error::error::ApiError;

pub async fn search(
    state: web::Data<State>,
    search_query: web::Json<SearchQuery>,
) -> Result<HttpResponse, ApiError> {
    log::info!(
        "Starting movie search with query: {:?}",
        search_query.q.clone()
    );

    let mut search_client = state.search_client.clone();
    let response = search_client.serch_movie(search_query.0.into()).await?;

    log::info!("Movie search completed successfully");

    Ok(HttpResponse::Ok().json(SearchResult::from(response)))
}
