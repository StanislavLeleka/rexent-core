use super::location_service::LocationServiceImpl;
use proto::services::location::{
    location_service_server::LocationService, GetCountriesWithCitiesRequest, GetLocationRequest,
    NewLocationRequest, UpdateLocationRequest,
};
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl LocationService for LocationServiceImpl {
    async fn add_location(
        &self,
        request: Request<NewLocationRequest>,
    ) -> Result<Response<proto::services::location::LocationResponse>, tonic::Status> {
        log::info!("Handling add_location request");

        let new_location_request = request.into_inner();
        let location = self.create_location(new_location_request).await?;

        log::info!("add_location request handled successfully");
        Ok(Response::new(location.into()))
    }

    async fn get_location(
        &self,
        request: Request<GetLocationRequest>,
    ) -> Result<Response<proto::services::location::LocationResponse>, Status> {
        log::info!("Handling get_location request");

        let get_location_request = request.into_inner();
        let location = self.get_location(get_location_request)?;

        log::info!("get_location request handled successfully");
        Ok(Response::new(location.into()))
    }
    async fn update_location(
        &self,
        request: Request<UpdateLocationRequest>,
    ) -> Result<Response<proto::services::location::LocationResponse>, Status> {
        log::info!("Handling update_location request");

        let update_location_request = request.into_inner();
        let location = self.update_location(update_location_request).await?;

        log::info!("update_location request handled successfully");
        Ok(Response::new(location.into()))
    }

    async fn get_countries_with_cities(
        &self,
        _: Request<GetCountriesWithCitiesRequest>,
    ) -> Result<Response<proto::services::location::CountriesResponse>, Status> {
        log::info!("Handling get_countries_with_cities request");

        let countries_with_cities = self.get_countries_with_cities()?;

        log::info!("get_countries_with_cities request handled successfully");
        Ok(Response::new(countries_with_cities.into()))
    }
}
