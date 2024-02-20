use crate::services::events::events_service_client::EventsServiceClient;
use crate::services::events::{GetShowtimesRequest, ShowtimesResponse};
use tonic::transport::Channel;
use tonic::Status;

#[derive(Clone)]
pub struct EventsServiceGrpcClient {
    client: EventsServiceClient<Channel>,
}

impl EventsServiceGrpcClient {
    pub async fn new(dst: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = EventsServiceClient::connect(dst).await?;
        Ok(Self { client })
    }

    pub async fn get_showtimes(
        &mut self,
        request: GetShowtimesRequest,
    ) -> Result<ShowtimesResponse, Status> {
        let response = self.client.get_showtimes(request).await?;
        Ok(response.into_inner())
    }
}
