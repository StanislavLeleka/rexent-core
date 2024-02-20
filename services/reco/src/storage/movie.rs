use db::mongo_repository::MongoRepository;
use mongodb::bson::doc;
use movie::models::country_movies::CountryMovies;

pub struct MovieStorage {
    pub repo: MongoRepository<CountryMovies>,
}

impl MovieStorage {
    pub fn new(repo: MongoRepository<CountryMovies>) -> Self {
        Self { repo }
    }

    pub async fn get_country_movies(
        &self,
        country: String,
    ) -> Result<Option<CountryMovies>, Box<dyn std::error::Error>> {
        log::info!("Getting movies for country: {}", country);

        let filter = doc! { "country": country.clone() };
        let country_movies = self.repo.find_one(filter, None).await?;

        println!("{:?}", country_movies);

        log::info!("Successfully received movies for country: {}", country);

        Ok(country_movies)
    }
}
