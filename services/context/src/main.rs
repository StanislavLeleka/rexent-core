use env_logger::Env;
use proto::{
    clients::location::LocationServiceGrpcClient,
    services::context::context_service_server::ContextServiceServer,
};
use redis::Client;
use services::context_impl::ContextServiceImpl;
use std::env;
use tonic::transport::Server;
use weatherapi::client::WeatherapiClient;

mod clients;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_logger();

    let context_service = get_context_service().await?;

    start_server(context_service).await?;

    Ok(())
}

fn initialize_logger() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
}

async fn get_context_service() -> Result<ContextServiceImpl, Box<dyn std::error::Error>> {
    let loc_url =
        env::var("LOCATION_SERVICE_GRPC_URL").expect("$LOCATION_SERVICE_GRPC_URL is not set");
    let loc_client = LocationServiceGrpcClient::new(loc_url).await.unwrap();
    let weather_client = WeatherapiClient::new().unwrap();
    let redis_url = env::var("REDIS_URL").expect("$REDIS_URL is not set");
    let redis_client = Client::open(redis_url).unwrap();

    let context_service = ContextServiceImpl::new(redis_client, loc_client, weather_client);

    Ok(context_service)
}

async fn start_server(
    context_service: ContextServiceImpl,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50052".parse()?;

    log::info!("Starting grpc server on [{:?}]", addr);

    Server::builder()
        .add_service(ContextServiceServer::new(context_service))
        .serve(addr)
        .await?;

    Ok(())
}
