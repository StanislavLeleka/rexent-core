use tonic::{transport::Channel, Request, Status};

use crate::services::users::{
    users_service_client::UsersServiceClient, GetUserRequest, LinkSocialAccountRequest,
    LoginRequest, NewUserRequest, TokenResponse, UserResponse,
};

#[derive(Clone)]
pub struct UserServiceGrpcClient {
    client: UsersServiceClient<Channel>,
}

impl UserServiceGrpcClient {
    pub async fn new(dst: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = UsersServiceClient::connect(dst).await?;
        Ok(Self { client })
    }

    pub async fn sign_up(&mut self, request: NewUserRequest) -> Result<TokenResponse, Status> {
        let response = self.client.sign_up(request).await?;
        Ok(response.into_inner())
    }

    pub async fn sign_in(&mut self, request: LoginRequest) -> Result<TokenResponse, Status> {
        let response = self.client.sign_in(request).await?;
        Ok(response.into_inner())
    }

    pub async fn get_user(&mut self, user_id: String) -> Result<UserResponse, Status> {
        let request = Request::new(GetUserRequest { user_id });
        let response = self.client.get_user(request).await?;
        Ok(response.into_inner())
    }

    pub async fn link_social_account(
        &mut self,
        request: LinkSocialAccountRequest,
    ) -> Result<(), Status> {
        let response = self.client.link_social_account(request).await?;
        Ok(response.into_inner())
    }
}
