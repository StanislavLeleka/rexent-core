use crate::services::events_service::EventsServiceImpl;
use db::{mongo, mongo_repository};
use env_logger::Env;
use proto::services::events::events_service_server::EventsServiceServer;
use tonic::transport::Server;

mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_logger();

    log::info!("Connecting to MongoDB");

    let client = mongo::connect().await?;
    let db = client.database("movies");
    let repo = mongo_repository::MongoRepository::new(&db, "loc_showtimes");

    let events_service = EventsServiceImpl::new(repo);
    start_server(events_service).await?;

    Ok(())
}

fn initialize_logger() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
}

async fn start_server(events_service: EventsServiceImpl) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50055".parse()?;

    log::info!("Starting grpc server on [{:?}]", addr);

    Server::builder()
        .add_service(EventsServiceServer::new(events_service))
        .serve(addr)
        .await?;

    Ok(())
}
