use std::env;

use proto::clients::events::EventsServiceGrpcClient;
use proto::clients::{
    context::ContextServiceGrpcClient, favorites::FavoritesServiceGrpcClient,
    location::LocationServiceGrpcClient, reco::RecoServiceGrpcClient,
    search::SearchServiceGrpcClient, users::UserServiceGrpcClient,
};

pub async fn create_users_client() -> Result<UserServiceGrpcClient, Box<dyn std::error::Error>> {
    let users_url = env::var("USER_SERVICE_GRPC_URL").expect("$USER_SERVICE_GRPC_URL is not set");
    log::info!("Connecting to user service at {}", users_url);

    let client = UserServiceGrpcClient::new(users_url).await?;
    log::info!("Successfully connected to user service");

    Ok(client)
}

pub async fn create_location_client(
) -> Result<LocationServiceGrpcClient, Box<dyn std::error::Error>> {
    let loc_url =
        env::var("LOCATION_SERVICE_GRPC_URL").expect("$LOCATION_SERVICE_GRPC_URL is not set");
    log::info!("Connecting to location service at {}", loc_url);

    let client = LocationServiceGrpcClient::new(loc_url).await?;
    log::info!("Successfully connected to location service");

    Ok(client)
}

pub async fn create_context_client() -> Result<ContextServiceGrpcClient, Box<dyn std::error::Error>>
{
    let context_url =
        env::var("CONTEXT_SERVICE_GRPC_URL").expect("$CONTEXT_SERVICE_GRPC_URL is not set");
    log::info!("Connecting to context service at {}", context_url);

    let client = ContextServiceGrpcClient::new(context_url).await?;
    log::info!("Successfully connected to context service");

    Ok(client)
}

pub async fn create_favorites_client(
) -> Result<FavoritesServiceGrpcClient, Box<dyn std::error::Error>> {
    let fav_url = env::var("USER_SERVICE_GRPC_URL").expect("$USER_SERVICE_GRPC_URL is not set");
    log::info!("Connecting to favorites service at {}", fav_url);

    let client = FavoritesServiceGrpcClient::new(fav_url).await?;
    log::info!("Successfully connected to favorites service");

    Ok(client)
}

pub async fn create_search_client() -> Result<SearchServiceGrpcClient, Box<dyn std::error::Error>> {
    let search_url =
        env::var("SEARCH_SERVICE_GRPC_URL").expect("$SEARCH_SERVICE_GRPC_URL is not set");
    log::info!("Connecting to search service at {}", search_url);

    let client = SearchServiceGrpcClient::new(search_url).await?;
    log::info!("Successfully connected to search service");

    Ok(client)
}

pub async fn create_reco_client() -> Result<RecoServiceGrpcClient, Box<dyn std::error::Error>> {
    let reco_url = env::var("RECO_SERVICE_GRPC_URL").expect("$RECO_SERVICE_GRPC_URL is not set");
    log::info!("Connecting to reco service at {}", reco_url);

    let client = RecoServiceGrpcClient::new(reco_url).await?;
    log::info!("Successfully connected to reco service");

    Ok(client)
}

pub async fn create_events_client() -> Result<EventsServiceGrpcClient, Box<dyn std::error::Error>> {
    let events_url =
        env::var("EVENTS_SERVICE_GRPC_URL").expect("$EVENTS_SERVICE_GRPC_URL is not set");
    log::info!("Connecting to events service at {}", events_url);

    let client = EventsServiceGrpcClient::new(events_url).await?;
    log::info!("Successfully connected to events service");

    Ok(client)
}
