use crawler::models::loc_showtimes::LocationShowtimes;
use db::mongo_repository::MongoRepository;
use mongodb::bson::doc;
use tonic::Status;

pub struct EventsServiceImpl {
    repo: MongoRepository<LocationShowtimes>,
}

impl EventsServiceImpl {
    pub fn new(repo: MongoRepository<LocationShowtimes>) -> Self {
        Self { repo }
    }

    pub async fn get_showtimes(
        &self,
        movie_id: i32,
        country: String,
        city: String,
    ) -> Result<Vec<LocationShowtimes>, Status> {
        log::info!("Getting showtimes");

        let query_filter = doc! {
            "$and": [
                {"movie.id": movie_id},
                {"loc.country_code": country},
                {"loc.city_code": city}
            ]
        };

        self.repo
            .find(query_filter, None)
            .await
            .map_err(|err| {
                log::info!("Error getting showtimes: {:?}", err);
                Status::internal("Failed to get showtimes")
            })
            .map(|showtimes| {
                log::info!("Found {} showtimes", showtimes.len());
                showtimes
            })
    }
}
