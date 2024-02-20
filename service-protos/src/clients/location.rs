use crate::services::location::{
    location_service_client::LocationServiceClient, CountriesResponse,
    GetCountriesWithCitiesRequest, GetLocationRequest, LocationResponse, NewLocationRequest,
    UpdateLocationRequest,
};
use tonic::{transport::Channel, Status};

#[derive(Clone)]
pub struct LocationServiceGrpcClient {
    client: LocationServiceClient<Channel>,
}

impl LocationServiceGrpcClient {
    pub async fn new(dst: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = LocationServiceClient::connect(dst).await?;
        Ok(Self { client })
    }

    pub async fn add_location(
        &mut self,
        request: NewLocationRequest,
    ) -> Result<LocationResponse, Status> {
        let response = self.client.add_location(request).await?;
        Ok(response.into_inner())
    }

    pub async fn get_location(
        &mut self,
        request: GetLocationRequest,
    ) -> Result<LocationResponse, Status> {
        let response = self.client.get_location(request).await?;
        Ok(response.into_inner())
    }

    pub async fn update_location(
        &mut self,
        request: UpdateLocationRequest,
    ) -> Result<LocationResponse, Status> {
        let response = self.client.update_location(request).await?;
        Ok(response.into_inner())
    }

    pub async fn get_countries_with_cities(
        &mut self,
        request: GetCountriesWithCitiesRequest,
    ) -> Result<CountriesResponse, Status> {
        let response = self.client.get_countries_with_cities(request).await?;
        Ok(response.into_inner())
    }
}
