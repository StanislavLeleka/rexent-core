use crate::models::country_movies::CountryMovies;
use db::{
    mongo,
    mongo_repository::{self, MongoRepository},
};
use mongodb::bson::{doc, DateTime};
use proto::{
    clients::location::LocationServiceGrpcClient,
    services::location::{CountryResponse, GetCountriesWithCitiesRequest},
};
use std::{collections::HashSet, env};
use tmdb_client::{
    models::movie::{Movie, MovieDetails},
    tmdb_client::TheMovieDbClient,
};

pub async fn fetch() -> Result<(), Box<dyn std::error::Error>> {
    let tmdb_client = TheMovieDbClient::new()?;
    let repo = get_mongo_repository("movies", "country_movies").await?;
    let mut loc_client = get_location_client().await?;

    log::info!("Getting a list of countries");
    let locations = loc_client
        .get_countries_with_cities(GetCountriesWithCitiesRequest {})
        .await?;

    log::info!(
        "Successfully received {} countries: {:#?}",
        locations.countries.len(),
        locations.countries
    );

    for country in locations.countries {
        if let Err(err) = process_country(&country, &tmdb_client, &repo).await {
            log::error!(
                "Error processing country [{}]: {:?}",
                country.country_name,
                err
            );
        }
    }

    Ok(())
}

async fn get_mongo_repository<T>(
    db_name: &str,
    collection: &str,
) -> Result<MongoRepository<T>, Box<dyn std::error::Error>> {
    let client = mongo::connect().await?;
    let db = client.database(db_name);

    Ok(mongo_repository::MongoRepository::new(&db, collection))
}

async fn get_location_client() -> Result<LocationServiceGrpcClient, Box<dyn std::error::Error>> {
    let loc_url =
        env::var("LOCATION_SERVICE_GRPC_URL").expect("$LOCATION_SERVICE_GRPC_URL is not set");
    let loc_client = LocationServiceGrpcClient::new(loc_url).await?;

    Ok(loc_client)
}

async fn process_country(
    country: &CountryResponse,
    tmdb_client: &TheMovieDbClient,
    repo: &MongoRepository<CountryMovies>,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Getting now playing movies for [{}]", country.country_name);

    let now_playing = tmdb_client
        .get_now_playing(country.country_code.to_lowercase(), "en-US".to_string(), 1)
        .await?;

    log::info!("Successfully received {} movies", now_playing.results.len());

    let country_movies = CountryMovies {
        country: country.country_code.clone(),
        start_date: to_date_time(&now_playing.dates.minimum)?,
        end_date: to_date_time(&now_playing.dates.maximum)?,
        movies: now_playing.results,
    };

    log::info!("Deleting existing movies for [{}]", country.country_name);
    repo.delete_many(
        doc! {
            "country": country.country_code.clone()
        },
        None,
    )
    .await?;

    log::info!("Saving movies for [{}]", country.country_name);
    repo.insert(&country_movies, None).await?;

    log::info!("Saving movie metadata for [{}]", country.country_name);
    save_movies_metadata(tmdb_client, country_movies.movies).await?;

    Ok(())
}

fn to_date_time(date: &str) -> Result<DateTime, Box<dyn std::error::Error>> {
    let date_time = DateTime::parse_rfc3339_str(&format!("{}T00:00:00.0Z", date))?;
    Ok(date_time)
}

async fn save_movies_metadata(
    tmdb_client: &TheMovieDbClient,
    movies: Vec<Movie>,
) -> Result<(), Box<dyn std::error::Error>> {
    let repo = get_mongo_repository("metadata", "movies_metadata").await?;
    let existing_ids = get_existing_movie_ids(&repo, &movies).await?;
    let new_movies = filter_new_movies(&movies, &existing_ids);

    let mut movie_details = vec![];

    for movie in new_movies {
        log::info!(
            "Getting metadata for movie [{}]",
            movie.title.as_ref().unwrap().clone()
        );
        let mut movie_detail = tmdb_client.get_movie_details(movie.id).await?;

        log::info!(
            "Getting credits and keywords for movie [{}]",
            movie.title.as_ref().unwrap().clone()
        );
        let movie_credits = tmdb_client.get_movie_credits(movie.id).await?;
        let movie_keywords = tmdb_client.get_movie_keywords(movie.id).await?;

        movie_detail.set_credits(movie_credits);
        movie_detail.set_keywords(movie_keywords);

        movie_details.push(movie_detail);
    }

    insert_new_movies(&repo, movie_details).await?;

    log::info!("Successfully saved movies metadata");
    Ok(())
}

async fn get_existing_movie_ids(
    repo: &MongoRepository<MovieDetails>,
    movies: &[Movie],
) -> Result<HashSet<i32>, Box<dyn std::error::Error>> {
    let filter = doc! {
        "id": {
            "$in": movies.iter().map(|m| m.id).collect::<Vec<i32>>()
        }
    };
    let existing_movies: Vec<MovieDetails> = repo.find(filter, None).await?;
    Ok(existing_movies.iter().map(|m| m.id).collect())
}

fn filter_new_movies(movies: &[Movie], existing_ids: &HashSet<i32>) -> Vec<Movie> {
    movies
        .iter()
        .filter(|m| !existing_ids.contains(&m.id))
        .cloned()
        .collect()
}

async fn insert_new_movies(
    repo: &MongoRepository<MovieDetails>,
    new_movies: Vec<MovieDetails>,
) -> Result<(), Box<dyn std::error::Error>> {
    repo.insert_many(&new_movies, None).await?;
    Ok(())
}
