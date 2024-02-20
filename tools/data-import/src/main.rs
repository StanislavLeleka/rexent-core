use db::{mongo, mongo_repository};
use env_logger::Env;
use models::models::Movie;
use movie_csv_migrator::read_movies_metadata;
use std::error::Error;
use tmdb_client::tmdb_client::TheMovieDbClient;
use tokio::time::{self, Duration};

mod aws_client;
mod models;
mod movie_csv_migrator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    load_metadata().await?;
    save_movies_metadata("out/movies_metadata_clear.csv".to_string()).await?;

    Ok(())
}

async fn load_metadata() -> Result<(), Box<dyn Error>> {
    let client = aws_client::get_client().await;
    let bucket = "neptune.moviesmetadata";
    let objects = vec![
        "cast_clear.csv",
        "keywords_clear.csv",
        "movies_metadata_clear.csv",
    ];

    for object in objects {
        log::info!("Downloading object: {}", object);
        aws_client::get_object(&client, bucket.to_string(), object.to_string()).await?;
        log::info!("Downloaded object: {}", object);
    }

    Ok(())
}

async fn save_movies_metadata(file_path: String) -> Result<(), Box<dyn Error>> {
    let mut movies = read_movies_metadata(file_path)?;

    let client = mongo::connect().await?;
    let db = client.database("metadata");
    let repo = mongo_repository::MongoRepository::new(&db, "movies_metadata");

    log::info!("Updating posters for movies");
    update_posters(&mut movies).await?;

    log::info!("Saving movies metadata to MongoDB");

    repo.insert_many(&movies, None).await?;

    log::info!("Successfully saved movies metadata to MongoDB");

    Ok(())
}

pub async fn update_posters(movies: &mut Vec<Movie>) -> Result<(), Box<dyn Error>> {
    let tmdb_client = TheMovieDbClient::new()?;
    let delay = Duration::from_millis(100);

    for movie in movies.iter_mut() {
        let movie_details = tmdb_client.get_movie_details(movie.id as i32).await;
        if let Ok(movie_details) = movie_details {
            if let Some(poster_path) = movie_details.poster_path {
                movie.poster_path = poster_path;
            }
        }
        time::sleep(delay).await;
    }

    Ok(())
}
