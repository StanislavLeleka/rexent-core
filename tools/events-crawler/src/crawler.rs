use crate::{
    clients::serp_api::SerpApiClient,
    models::{location::Location, showtime::Showtime},
};
use std::env;

pub async fn crawl(
    movie: &String,
    loc: &Location,
) -> Result<Vec<Showtime>, Box<dyn std::error::Error>> {
    log::info!(
        "Start google crawling for movie [{}] in city [{}]",
        movie,
        loc.city_name.clone()
    );

    let api_key = env::var("SERPAPI_API_KEY").expect("$SERPAPI_API_KEY is not set");
    let mut client = SerpApiClient::new();

    set_search_params(&mut client, movie, loc);

    let showtimes = client.showtimes(api_key).await?;
    Ok(showtimes)
}

fn set_search_params(client: &mut SerpApiClient, movie: &str, loc: &Location) {
    log::info!("Setting search params");

    client
        .set_engine("google".to_owned())
        .set_param("q", format!("{} showtimes", movie))
        .set_param("gl", loc.country_code.clone())
        .set_param(
            "location",
            format!("{} {}", loc.city_name, loc.country_name),
        );
}
