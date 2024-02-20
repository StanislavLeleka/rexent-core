use env_logger::Env;
use events_crawler::{event_processor, models::location::Location};
use proto::{
    clients::location::LocationServiceGrpcClient, services::location::GetCountriesWithCitiesRequest,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let locations = get_locations().await?;

    event_processor::process_locations(locations).await;

    Ok(())
}

async fn get_locations() -> Result<Vec<Location>, Box<dyn std::error::Error>> {
    let mut loc_client = get_location_client().await?;

    log::info!("Getting a list of countries");
    let countries_response = loc_client
        .get_countries_with_cities(GetCountriesWithCitiesRequest {})
        .await?;

    log::info!(
        "Successfully received {} countries: {:#?}",
        countries_response.countries.len(),
        countries_response.countries
    );

    let locations = countries_response
        .countries
        .iter()
        .flat_map(|country| {
            country.cities.iter().map(move |city| Location {
                city_code: city.city_code.clone(),
                city_name: city.city_name.clone(),
                country_code: country.country_code.clone(),
                country_name: country.country_name.clone(),
            })
        })
        .collect::<Vec<Location>>();

    Ok(locations)
}

async fn get_location_client() -> Result<LocationServiceGrpcClient, Box<dyn std::error::Error>> {
    let loc_url =
        env::var("LOCATION_SERVICE_GRPC_URL").expect("$LOCATION_SERVICE_GRPC_URL is not set");
    let loc_client = LocationServiceGrpcClient::new(loc_url).await?;

    Ok(loc_client)
}
