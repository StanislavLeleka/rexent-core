use crate::services::context::{
    context_service_client::ContextServiceClient, ContextResponse, GetContextRequest,
    NewContextRequest,
};
use tonic::{transport::Channel, Status};

#[derive(Clone)]
pub struct ContextServiceGrpcClient {
    client: ContextServiceClient<Channel>,
}

impl ContextServiceGrpcClient {
    pub async fn new(dst: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = ContextServiceClient::connect(dst).await?;
        Ok(Self { client })
    }

    pub async fn create_context(
        &mut self,
        request: NewContextRequest,
    ) -> Result<ContextResponse, Status> {
        let response = self.client.create_context(request).await?;
        Ok(response.into_inner())
    }

    pub async fn get_context(
        &mut self,
        request: GetContextRequest,
    ) -> Result<ContextResponse, Status> {
        let response = self.client.get_context(request).await?;
        Ok(response.into_inner())
    }
}
