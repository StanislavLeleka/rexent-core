use crate::services::{favorites_service::FavoritesServiceImpl, users_service::UserService};
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};
use env_logger::Env;
use proto::services::{
    favorites::favorites_service_server::FavoritesServiceServer,
    users::users_service_server::UsersServiceServer,
};
use std::env;
use tonic::transport::Server;

mod models;
mod schema;
mod services;
mod utils;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_logger();

    let pool = db_connect()?;

    let user_service = UserService::new(pool.clone());
    let fav_service = FavoritesServiceImpl::new(pool);

    start_server(user_service, fav_service).await?;

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
    user_service: UserService,
    fav_service: FavoritesServiceImpl,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50050".parse()?;

    log::info!("Starting grpc server on [{:?}]", addr);

    Server::builder()
        .add_service(UsersServiceServer::new(user_service))
        .add_service(FavoritesServiceServer::new(fav_service))
        .serve(addr)
        .await?;

    Ok(())
}
