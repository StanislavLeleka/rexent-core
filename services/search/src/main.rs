use db::{mongo, mongo_repository};
use env_logger::Env;
use proto::services::search::search_service_server::SearchServiceServer;
use services::search_service::SearchServiceImpl;
use tonic::transport::Server;

pub mod models;
pub mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_logger();

    log::info!("Connecting to MongoDB");

    let client = mongo::connect().await?;
    let db = client.database("metadata");
    let repo = mongo_repository::MongoRepository::new(&db, "movies_metadata");

    let search_service = SearchServiceImpl::new(repo);

    start_server(search_service).await?;

    Ok(())
}

fn initialize_logger() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
}

async fn start_server(search_service: SearchServiceImpl) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50053".parse()?;

    log::info!("Starting grpc server on [{:?}]", addr);

    Server::builder()
        .add_service(SearchServiceServer::new(search_service))
        .serve(addr)
        .await?;

    Ok(())
}
