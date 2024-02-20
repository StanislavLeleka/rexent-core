use clients::geolocation::Geolocation;
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};
use env_logger::Env;
use http_client::http_client::HttpClient;
use proto::services::location::location_service_server::LocationServiceServer;
use services::location_service::LocationServiceImpl;
use std::env;
use tonic::transport::Server;

mod aws;
mod clients;
mod models;
mod schema;
mod services;
mod utils;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_logger();

    let pool = db_connect()?;
    let geolocation = Geolocation::new(HttpClient::new());

    let location_service = LocationServiceImpl::new(pool, geolocation);

    start_server(location_service).await?;

    Ok(())
}

fn initialize_logger() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
}

pub fn db_connect() -> Result<DbPool, PoolError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder().build(manager).map_err(|err| {
        log::error!("Failed to connect to database: {:?}", err);
        err
    })
}

async fn start_server(
    location_service: LocationServiceImpl,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    log::info!("Starting grpc server on [{:?}]", addr);

    Server::builder()
        .add_service(LocationServiceServer::new(location_service))
        .serve(addr)
        .await?;

    Ok(())
}
