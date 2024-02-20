use crate::services::favorites::{
    favorites_service_client::FavoritesServiceClient, AddLikedMoviesRequest,
    AddLikedMoviesResponse, GetLikedMoviesRequest, GetLikedMoviesResponse,
};
use tonic::{transport::Channel, Status};

#[derive(Clone)]
pub struct FavoritesServiceGrpcClient {
    client: FavoritesServiceClient<Channel>,
}

impl FavoritesServiceGrpcClient {
    pub async fn new(dst: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = FavoritesServiceClient::connect(dst).await?;
        Ok(Self { client })
    }

    pub async fn add_liked_movies(
        &mut self,
        request: AddLikedMoviesRequest,
    ) -> Result<AddLikedMoviesResponse, Status> {
        let response = self.client.add_liked_movies(request).await?;
        Ok(response.into_inner())
    }

    pub async fn get_liked_movies(
        &mut self,
        request: GetLikedMoviesRequest,
    ) -> Result<GetLikedMoviesResponse, Status> {
        let response = self.client.get_liked_movies(request).await?;
        Ok(response.into_inner())
    }
}
