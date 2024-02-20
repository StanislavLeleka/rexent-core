use crate::models::mappers::to_grpc;
use crate::services::events_service::EventsServiceImpl;
use proto::services::events::events_service_server::EventsService;
use proto::services::events::{GetShowtimesRequest, ShowtimesResponse};
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl EventsService for EventsServiceImpl {
    async fn get_showtimes(
        &self,
        request: Request<GetShowtimesRequest>,
    ) -> Result<Response<ShowtimesResponse>, Status> {
        log::info!("Handling get_showtimes request");

        let get_showtimes_request = request.into_inner();
        let showtimes = self
            .get_showtimes(
                get_showtimes_request.movie_id,
                get_showtimes_request.country,
                get_showtimes_request.city,
            )
            .await?;

        log::info!("get_showtimes request handled successfully");

        Ok(Response::new(ShowtimesResponse {
            showtimes: showtimes
                .into_iter()
                .flat_map(|s| s.showtimes)
                .map(|s| to_grpc(s))
                .collect(),
        }))
    }
}
