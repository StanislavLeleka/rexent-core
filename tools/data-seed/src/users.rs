use async_trait::async_trait;
use log::log;
use proto::clients::location::LocationServiceGrpcClient;
use proto::clients::users::UserServiceGrpcClient;
use proto::services::location::NewLocationRequest;
use proto::services::users::{NewUserRequest, TokenResponse};
use std::env;
use uuid::Uuid;

fn get_env_var(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    env::var(key).map_err(|_| Box::from(format!("${} is not set", key)))
}

async fn create_client<T: CreateClient>(url_key: &str) -> Result<T, Box<dyn std::error::Error>> {
    let url = get_env_var(url_key)?;
    log::info!("Connecting to service at {}", url);

    let client = T::new_client(url).await?;
    log::info!("Successfully connected to service");

    Ok(client)
}

#[async_trait]
trait CreateClient {
    async fn new_client(url: String) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

#[async_trait]
impl CreateClient for UserServiceGrpcClient {
    async fn new_client(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        Self::new(url).await
    }
}

#[async_trait]
impl CreateClient for LocationServiceGrpcClient {
    async fn new_client(url: String) -> Result<Self, Box<dyn std::error::Error>> {
        Self::new(url).await
    }
}

pub async fn create_users(
    count: i32,
    city: String,
    lat: f32,
    lng: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut users_client = create_client::<UserServiceGrpcClient>("USER_SERVICE_GRPC_URL").await?;
    let mut loc_client =
        create_client::<LocationServiceGrpcClient>("LOCATION_SERVICE_GRPC_URL").await?;

    log::info!("Creating {} users in {}", count, city);
    for i in 0..count {
        let user = NewUserRequest {
            email: format!("test_user+{}@mail.com", Uuid::new_v4()),
            password: "qwerty".to_string(),
            first_name: "First".to_string(),
            last_name: "Last".to_string(),
        };

        let token_response = users_client.sign_up(user).await?;
        set_user_location(&mut loc_client, token_response, lat, lng).await?;

        log::info!("Created user {} of {}", i + 1, count);
    }

    Ok(())
}

async fn set_user_location(
    loc_client: &mut LocationServiceGrpcClient,
    token_response: TokenResponse,
    lat: f32,
    lng: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Setting user location");
    let new_location = NewLocationRequest {
        user_id: token_response.user_id,
        lat,
        lng,
    };

    loc_client.add_location(new_location).await?;
    log::info!("Successfully set user location");

    Ok(())
}
