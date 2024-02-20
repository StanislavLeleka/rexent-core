use db::{
    mongo,
    mongo_repository::{self},
    postgres::DbPool,
};
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};
use env_logger::Env;
use movie_reco_client::movie_reco_client::MovieRecoClient;
use proto::{
    clients::{context::ContextServiceGrpcClient, favorites::FavoritesServiceGrpcClient},
    services::reco::reco_service_server::RecoServiceServer,
};
use recommender::movie::MovieRecommender;
use services::reco_impl::RecoServiceImpl;
use std::env;
use storage::movie::MovieStorage;
use tonic::transport::Server;

pub mod models;
pub mod recommender;
pub mod schema;
pub mod services;
pub mod storage;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_logger();

    log::info!("Connecting to MongoDB");
    let reco_service = get_reco_service().await?;
    log::info!("Connected to MongoDB");

    start_server(reco_service).await?;

    Ok(())
}

fn initialize_logger() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
}

async fn get_reco_service() -> Result<RecoServiceImpl, Box<dyn std::error::Error>> {
    let movie_reco_client = get_movie_reco_client();
    let favorites_client = get_favorites_client().await?;
    let context_client = get_context_client().await?;
    let movie_storage = get_movie_storage().await?;

    let movie_recommender = MovieRecommender::new(
        movie_reco_client,
        favorites_client,
        context_client,
        movie_storage,
    );

    let pool = db_connect()?;

    let context_service = RecoServiceImpl {
        movie_recommender,
        db: pool,
    };

    Ok(context_service)
}

fn get_movie_reco_client() -> MovieRecoClient {
    MovieRecoClient::new()
}

async fn get_favorites_client() -> Result<FavoritesServiceGrpcClient, Box<dyn std::error::Error>> {
    let fav_url = env::var("FAV_SERVICE_GRPC_URL").expect("$FAV_SERVICE_GRPC_URL is not set");
    FavoritesServiceGrpcClient::new(fav_url).await
}

async fn get_context_client() -> Result<ContextServiceGrpcClient, Box<dyn std::error::Error>> {
    let context_url =
        env::var("CONTEXT_SERVICE_GRPC_URL").expect("$CONTEXT_SERVICE_GRPC_URL is not set");
    ContextServiceGrpcClient::new(context_url).await
}

async fn get_movie_storage() -> Result<MovieStorage, Box<dyn std::error::Error>> {
    let client = mongo::connect().await?;
    let db = client.database("movies");
    let repo = mongo_repository::MongoRepository::new(&db, "country_movies");
    Ok(MovieStorage::new(repo))
}

pub fn db_connect() -> Result<DbPool, PoolError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder().build(manager).map_err(|err| {
        log::error!("Failed to connect to database: {:?}", err);
        err
    })
}

async fn start_server(reco_service: RecoServiceImpl) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50054".parse()?;

    log::info!("Starting grpc server on [{:?}]", addr);

    Server::builder()
        .add_service(RecoServiceServer::new(reco_service))
        .serve(addr)
        .await?;

    log::info!("Started grpc server on [{:?}]", addr);

    Ok(())
}
