use super::context_impl::ContextServiceImpl;
use proto::services::context::{
    context_service_server::ContextService, ContextResponse, GetContextRequest, NewContextRequest,
};
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl ContextService for ContextServiceImpl {
    async fn create_context(
        &self,
        request: Request<NewContextRequest>,
    ) -> Result<Response<ContextResponse>, Status> {
        log::info!("Handling create_context request");

        let new_context_request = request.into_inner();
        let context = self.create(new_context_request).await?;

        log::info!("create_context request handled successfully");

        Ok(Response::new(context.into()))
    }

    async fn get_context(
        &self,
        request: Request<GetContextRequest>,
    ) -> Result<Response<ContextResponse>, Status> {
        log::info!("Handling get_context request");

        let get_context_request = request.into_inner();
        let context = self.get(get_context_request).await?;

        log::info!("get_context request handled successfully");

        Ok(Response::new(context.into()))
    }
}
