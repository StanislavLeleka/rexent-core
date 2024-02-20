use crate::models::models::Movie;
use db::mongo_repository::MongoRepository;
use mongodb::{bson::doc, options::FindOptions};
use proto::services::search::SearchMovieRequest;
use tonic::Status;

const DEFAULT_PAGE: i64 = 0;
const DEFAULT_PAGE_SIZE: i64 = 10;

pub struct SearchServiceImpl {
    repo: MongoRepository<Movie>,
}

impl SearchServiceImpl {
    pub fn new(repo: MongoRepository<Movie>) -> Self {
        Self { repo }
    }

    pub(super) async fn search(&self, request: SearchMovieRequest) -> Result<Vec<Movie>, Status> {
        let page = request.page.unwrap_or(DEFAULT_PAGE);
        let page_size = request.page_size.unwrap_or(DEFAULT_PAGE_SIZE);

        log::info!("Searching for movies with query: {}", request.query);
        log::debug!("Using page: {}, page_size: {}", page, page_size);

        let query_filter = doc! {"original_title": {"$regex": &request.query, "$options": "i"}};
        let find_options = FindOptions::builder()
            .skip((page * page_size) as u64)
            .limit(page_size as i64)
            .sort(doc! {"vote_count": -1})
            .build();

        self.repo
            .find(query_filter, Some(find_options))
            .await
            .map_err(|err| {
                log::info!("Error searching movies: {:?}", err);
                Status::internal("Failed to search movies")
            })
            .map(|movies| {
                log::info!("Found {} movies", movies.len());
                movies
            })
    }
}
