use crate::services::search::{
    search_service_client::SearchServiceClient, SearchMovieRequest, SearchResultResponse,
};
use tonic::{transport::Channel, Status};

#[derive(Clone)]
pub struct SearchServiceGrpcClient {
    client: SearchServiceClient<Channel>,
}

impl SearchServiceGrpcClient {
    pub async fn new(dst: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = SearchServiceClient::connect(dst).await?;
        Ok(Self { client })
    }

    pub async fn serch_movie(
        &mut self,
        request: SearchMovieRequest,
    ) -> Result<SearchResultResponse, Status> {
        let response = self.client.search_movie(request).await?;
        Ok(response.into_inner())
    }
}
