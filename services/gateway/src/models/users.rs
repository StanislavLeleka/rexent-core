use actix_web::web::Form;
use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct NewUserRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub user_id: String,
    pub access_token: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Deserialize)]
pub struct NewSocialAccountRequest {
    pub user_id: Option<String>,
    pub id: String,
    pub platform: String,
    pub account_name: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
}

impl From<LoginRequest> for proto::services::users::LoginRequest {
    fn from(value: LoginRequest) -> Self {
        proto::services::users::LoginRequest {
            email: value.email,
            password: value.password,
        }
    }
}

impl From<NewUserRequest> for proto::services::users::NewUserRequest {
    fn from(value: NewUserRequest) -> Self {
        proto::services::users::NewUserRequest {
            email: value.email,
            password: value.password,
            first_name: value.first_name,
            last_name: value.last_name,
        }
    }
}

impl From<NewSocialAccountRequest> for proto::services::users::LinkSocialAccountRequest {
    fn from(value: NewSocialAccountRequest) -> Self {
        proto::services::users::LinkSocialAccountRequest {
            user_id: value.user_id.unwrap(),
            id: value.id,
            platform: value.platform,
            account_name: value.account_name,
            access_token: value.access_token,
            refresh_token: value.refresh_token,
            expires_at: Utc
                .timestamp_opt(value.expires_at, 0)
                .unwrap()
                .naive_utc()
                .to_string(),
        }
    }
}

impl From<proto::services::users::TokenResponse> for TokenResponse {
    fn from(value: proto::services::users::TokenResponse) -> Self {
        TokenResponse {
            user_id: value.user_id,
            access_token: value.access_token,
        }
    }
}

impl From<proto::services::users::UserResponse> for UserResponse {
    fn from(value: proto::services::users::UserResponse) -> Self {
        UserResponse {
            user_id: value.user_id,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
        }
    }
}
