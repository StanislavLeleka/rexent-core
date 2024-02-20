use super::search_service::SearchServiceImpl;
use proto::services::search::{
    search_service_server::SearchService, SearchMovieRequest, SearchResultResponse,
};
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl SearchService for SearchServiceImpl {
    async fn search_movie(
        &self,
        request: Request<SearchMovieRequest>,
    ) -> Result<Response<SearchResultResponse>, Status> {
        log::info!("Handling search_movie request");

        let search_movie_request = request.into_inner();
        let movies = self.search(search_movie_request).await?;

        log::info!("Search returned {} movies", movies.len());
        let response_movies = movies.iter().map(|m| m.to_owned().into()).collect();

        log::info!("search_movie request handled successfully");

        Ok(Response::new(SearchResultResponse {
            movies: response_movies,
        }))
    }
}
