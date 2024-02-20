use crate::grpc_factory::create_events_client;
use actix_web::{http::Method, middleware::Logger, App, HttpServer};
use auth::http::{auth_middleware::UNAUTHORIZED_ROUTES, authentication::Authentication};
use env_logger::Env;
use grpc_factory::{
    create_context_client, create_favorites_client, create_location_client, create_reco_client,
    create_search_client, create_users_client,
};
use middleware::state::State;

mod api;
mod grpc_factory;
mod middleware;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("Starting gateway service...");

    let state = build_state().await;

    add_unauthorized_routes();

    start_server(state).await?;

    Ok(())
}

async fn build_state() -> State {
    let users_client = create_users_client().await.unwrap();
    let loc_client = create_location_client().await.unwrap();
    let context_client = create_context_client().await.unwrap();
    let favorites_client = create_favorites_client().await.unwrap();
    let search_client = create_search_client().await.unwrap();
    let reco_client = create_reco_client().await.unwrap();
    let events_client = create_events_client().await.unwrap();

    State {
        users_client,
        loc_client,
        context_client,
        search_client,
        favorites_client,
        reco_client,
        events_client,
    }
}

async fn start_server(state: State) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Authentication)
            .app_data(actix_web::web::Data::new(state.clone()))
            .configure(routes::api)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn add_unauthorized_routes() {
    log::info!("Adding unauthorized routes");

    let mut unauthorized_routes = UNAUTHORIZED_ROUTES.lock().unwrap();
    unauthorized_routes.add_route("/api/users/signup", Method::POST);
    unauthorized_routes.add_route("/api/users/signin", Method::POST);
    unauthorized_routes.add_route("/api/reco/types", Method::GET);

    log::info!("Unauthorized routes added successfully");
}
