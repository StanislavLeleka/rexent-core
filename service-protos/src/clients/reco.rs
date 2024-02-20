use crate::services::reco::{
    reco_service_client::RecoServiceClient, AddUserRecommendationTypesRequest,
    AddUserRecommendationTypesResponse, GetMovieRecommendationsRequest,
    GetMovieRecommendationsResponse, GetRecommendationTypesRequest, GetRecommendationTypesResponse,
    GetUserRecommendationTypesRequest, GetUserRecommendationTypesResponse,
};
use tonic::{transport::Channel, Status};

#[derive(Clone)]
pub struct RecoServiceGrpcClient {
    client: RecoServiceClient<Channel>,
}

impl RecoServiceGrpcClient {
    pub async fn new(dst: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = RecoServiceClient::connect(dst).await?;
        Ok(Self { client })
    }

    pub async fn get_movie_recommendations(
        &mut self,
        request: GetMovieRecommendationsRequest,
    ) -> Result<GetMovieRecommendationsResponse, Status> {
        let response = self.client.get_movie_recommendations(request).await?;
        Ok(response.into_inner())
    }

    pub async fn add_user_recommendation_types(
        &mut self,
        request: AddUserRecommendationTypesRequest,
    ) -> Result<AddUserRecommendationTypesResponse, Status> {
        let response = self.client.add_user_recommendation_types(request).await?;
        Ok(response.into_inner())
    }

    pub async fn get_user_recommendation_types(
        &mut self,
        request: GetUserRecommendationTypesRequest,
    ) -> Result<GetUserRecommendationTypesResponse, Status> {
        let response = self.client.get_user_recommendation_types(request).await?;
        Ok(response.into_inner())
    }
    pub async fn get_recommendation_types(
        &mut self,
        request: GetRecommendationTypesRequest,
    ) -> Result<GetRecommendationTypesResponse, Status> {
        let response = self.client.get_recommendation_types(request).await?;
        Ok(response.into_inner())
    }
}
