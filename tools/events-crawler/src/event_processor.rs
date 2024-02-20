use crate::{
    crawler,
    models::{loc_showtimes::LocationShowtimes, location::Location, showtime::Showtime},
};
use chrono::Utc;
use db::{
    mongo,
    mongo_repository::{self, MongoRepository},
};
use mongodb::{
    bson::{doc, DateTime},
    results::InsertOneResult,
};
use movie_fetcher::models::country_movies::CountryMovies;
use std::error::Error;
use tmdb_client::models::movie::Movie;

const MOVIES_LIMIT: usize = 2;

pub async fn process_locations(locations: Vec<Location>) {
    let movie_repo = get_mongo_repository::<CountryMovies>("country_movies")
        .await
        .unwrap();
    let showtime_repo = get_mongo_repository::<LocationShowtimes>("loc_showtimes")
        .await
        .unwrap();

    for loc in locations {
        if let Ok(country_movies) = get_country_movies(&movie_repo, &loc).await {
            process_movies(&country_movies.movies, &showtime_repo, &loc).await;
        }
    }
}

async fn get_country_movies(
    movie_repo: &MongoRepository<CountryMovies>,
    loc: &Location,
) -> Result<CountryMovies, Box<dyn Error>> {
    let filter = doc! {"country": loc.country_code.clone()};

    match movie_repo.find_one(filter.clone(), None).await {
        Ok(Some(movies)) => Ok(movies),
        Ok(None) => {
            if movie_fetcher::fetcher::fetch().await.is_ok() {
                match movie_repo.find_one(filter, None).await {
                    Ok(Some(new_movies)) => Ok(new_movies),
                    Ok(None) => {
                        log::warn!(
                            "No movies found after fetching for country: {}",
                            loc.country_name
                        );
                        Err("No movies found after fetching".into())
                    }
                    Err(err) => {
                        log::warn!("Error fetching new country movies: {:?}", err);
                        Err(err.into())
                    }
                }
            } else {
                log::warn!("Failed to fetch movies");
                Err("Failed to fetch movies".into())
            }
        }
        Err(err) => {
            log::info!("Error during initial movies retrieval: {}", err);
            Err(err.into())
        }
    }
}

async fn process_movies(
    movies: &[Movie],
    repo: &MongoRepository<LocationShowtimes>,
    loc: &Location,
) {
    for movie in movies.iter().take(MOVIES_LIMIT) {
        let title = movie.original_title.as_ref().unwrap();
        match crawler::crawl(&title, loc).await {
            Ok(showtimes) => {
                if let Err(err) = save_showtimes(showtimes, repo, movie.clone(), loc).await {
                    log::error!("Error during saving showtimes: {}", err);
                }
            }
            Err(err) => {
                log::info!(
                    "Error crawling movie [{}] showtimes at location [{}]: {}",
                    title,
                    loc.city_name,
                    err
                );
            }
        }
    }
}

async fn save_showtimes(
    showtimes: Vec<Showtime>,
    repo: &MongoRepository<LocationShowtimes>,
    movie: Movie,
    loc: &Location,
) -> Result<InsertOneResult, Box<dyn std::error::Error>> {
    let loc_showtimes = LocationShowtimes {
        movie,
        loc: loc.clone(),
        showtimes,
    };

    repo.insert(&loc_showtimes, None).await.map_err(|err| {
        log::info!("Error during saving showtimes");
        err.into()
    })
}

async fn get_mongo_repository<T>(
    collection_name: &str,
) -> Result<MongoRepository<T>, Box<dyn std::error::Error>> {
    let client = mongo::connect().await?;
    let db = client.database("movies");

    Ok(mongo_repository::MongoRepository::new(&db, collection_name))
}
