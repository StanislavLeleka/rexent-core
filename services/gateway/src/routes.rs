use actix_web::web::{self, get, post, put, ServiceConfig};

use crate::api;

// Configure the users endpoints
fn configure_users(config: &mut ServiceConfig) {
    config.service(
        web::scope("/users")
            .route("/signup", web::post().to(api::users::sign_up))
            .route("/signin", web::post().to(api::users::sign_in))
            .route("/me", web::get().to(api::users::get_user))
            .route("/social", web::post().to(api::users::link_social_account))
            .route(
                "/fav/movies",
                web::post().to(api::favorites::add_liked_movies),
            )
            .route(
                "/fav/movies/{user_id}",
                web::get().to(api::favorites::get_liked_movies),
            ),
    );
}

// Configure the locations endpoints
fn configure_locations(config: &mut ServiceConfig) {
    config.service(
        web::scope("/locations")
            .route("", web::post().to(api::location::add_location))
            .route(
                "/countries",
                web::get().to(api::location::get_countries_with_cities),
            )
            .route("/{user_id}", web::get().to(api::location::get_location))
            .route("/{user_id}", web::put().to(api::location::update_location)),
    );
}

// Configure the context endpoints
fn configure_context(config: &mut ServiceConfig) {
    config.service(
        web::scope("/context")
            .route("", web::post().to(api::context::create_context))
            .route("/{user_id}", web::get().to(api::context::get_context)),
    );
}

// Configure the search endpoints
fn configure_search(config: &mut ServiceConfig) {
    config.service(web::scope("/search/movies").route("", post().to(api::search::search)));
}

// Configure the reco endpoints
fn configure_reco(config: &mut ServiceConfig) {
    config.service(
        web::scope("/reco")
            .route(
                "/movies/{user_id}",
                web::get().to(api::reco::get_movie_recommendations),
            )
            .route(
                "/types/user/{user_id}",
                web::post().to(api::reco::add_user_recommendation_types),
            )
            .route(
                "/types/user/{user_id}",
                web::get().to(api::reco::get_user_recommendation_types),
            )
            .route("/types", web::get().to(api::reco::get_recommendation_types)),
    );
}

// Configure the events endpoints
fn configure_events(config: &mut ServiceConfig) {
    config.service(web::scope("/events").route("", post().to(api::events::get_showtimes)));
}

// Main API configuration
pub fn api(config: &mut ServiceConfig) {
    config.service(
        web::scope("/api")
            .configure(configure_users)
            .configure(configure_locations)
            .configure(configure_context)
            .configure(configure_search)
            .configure(configure_reco)
            .configure(configure_events),
    );
}
