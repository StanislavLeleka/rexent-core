use crate::models::users::NewSocialAccount;
use crate::schema::user_social_accounts;
use crate::{
    models::users::{NewUser, User},
    schema::users,
    utils::{
        db::get_db_conn,
        hasher::{self, hash_password},
        token,
    },
    DbPool,
};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use log::log;
use proto::services::users::{
    users_service_server::UsersService, GetUserRequest, LinkSocialAccountRequest, LoginRequest,
    NewUserRequest, TokenResponse, UserResponse,
};
use std::str::FromStr;
use tonic::{Request, Response, Status};

pub struct UserService {
    db: DbPool,
}

impl UserService {
    pub fn new(db: DbPool) -> Self {
        log::info!("Initializing UserService");
        Self { db }
    }

    fn create_user(&self, request: NewUserRequest) -> Result<User, Status> {
        log::info!("Creating user with email: {}", request.email.clone());

        let conn = &mut get_db_conn(self.db.clone())?;
        let user_record = conn
            .transaction::<_, Box<dyn std::error::Error>, _>(|conn| {
                let password_hash = hash_password(&request.password)?;
                let new_user = NewUser {
                    id: uuid::Uuid::new_v4().to_string(),
                    email: request.email.clone(),
                    pwd_hash: password_hash,
                    first_name: request.first_name,
                    last_name: request.last_name,
                };

                let user_record = diesel::insert_into(users::table)
                    .values(&new_user)
                    .get_result::<User>(conn)?;

                log::info!("User created successfully with email: {}", request.email);
                Ok(user_record)
            })
            .map_err(|err| {
                log::error!("Error creating user: {:?}", err);
                Status::internal("Failed to create user")
            })?;

        Ok(user_record)
    }

    fn get_user_by_email(&self, email: &str) -> Result<User, Status> {
        log::info!("Getting user by email: {}", email);

        let conn = &mut get_db_conn(self.db.clone())?;
        let user_record = diesel::QueryDsl::filter(users::table, users::email.eq(email))
            .limit(1)
            .first::<User>(conn)
            .map_err(|err| {
                log::error!("Error getting user by email: {:?}", err);
                Status::internal("Failed to get user by email")
            })?;

        log::info!("User retrieved by email: {}", email);
        Ok(user_record)
    }

    fn get_user_by_id(&self, user_id: &str) -> Result<User, Status> {
        log::info!("Getting user by ID: {}", user_id);

        let conn = &mut get_db_conn(self.db.clone())?;
        let user_record = users::table
            .filter(users::id.eq(user_id))
            .limit(1)
            .first::<User>(conn)
            .map_err(|err| {
                log::error!("Error getting user by id: {:?}", err);
                Status::internal("Failed to get user by id")
            })?;

        log::info!("User retrieved by ID: {}", user_id);
        Ok(user_record)
    }

    fn link_social_account(&self, request: LinkSocialAccountRequest) -> Result<(), Status> {
        log::info!("Linking social account for user ID: {}", request.user_id);

        log::info!("request: {:?}", request);

        let conn = &mut get_db_conn(self.db.clone())?;
        conn.transaction::<_, Box<dyn std::error::Error>, _>(|conn| {
            let social_account = NewSocialAccount {
                id: request.id,
                user_id: request.user_id.clone(),
                platform: request.platform.clone(),
                account_name: request.account_name.clone(),
                access_token: request.access_token.clone(),
                refresh_token: request.refresh_token.clone(),
                expires_at: NaiveDateTime::parse_from_str(
                    request.expires_at.as_str(),
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap(),
            };

            diesel::insert_into(user_social_accounts::table)
                .values(&social_account)
                .execute(conn)?;

            log::info!(
                "Social account linked successfully for user ID: {}",
                request.user_id
            );
            Ok(())
        })
        .map_err(|err| {
            log::error!("Error linking social account: {:?}", err);
            Status::internal("Failed to link social account")
        })
    }

    fn generate_token(&self, user_record: &User) -> Result<String, Status> {
        log::debug!("Generating token for user ID: {}", user_record.id.clone());

        let now = Utc::now().timestamp_nanos() / 1_000_000_000;
        let token = token::generate(user_record.id.clone(), now).map_err(|err| {
            log::error!("Error generating token: {:?}", err);
            Status::internal("Failed to generate token")
        })?;

        log::debug!("Token generated successfully");
        Ok(token)
    }

    fn verify_password(&self, user_record: &User, pwd: &str) -> Result<bool, bcrypt::BcryptError> {
        log::debug!("Verifying password for user ID: {}", user_record.id);
        hasher::verify(pwd, &user_record.pwd_hash)
    }
}

#[tonic::async_trait]
impl UsersService for UserService {
    async fn sign_up(
        &self,
        request: Request<NewUserRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        log::info!("Handling sign_up request");

        let new_user_request = request.into_inner();
        let user_record = self.create_user(new_user_request)?;
        let access_token = self.generate_token(&user_record)?;

        log::info!("sign_up request handled successfully");
        Ok(Response::new(TokenResponse {
            access_token,
            user_id: user_record.id,
        }))
    }

    async fn sign_in(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        log::info!("Handling sign_in request");

        let login_request = request.into_inner();
        let user_record = self.get_user_by_email(&login_request.email)?;

        if let Ok(verified) = self.verify_password(&user_record, &login_request.password) {
            if verified {
                let access_token = self.generate_token(&user_record)?;

                log::info!("sign_in request handled successfully");
                return Ok(Response::new(TokenResponse {
                    access_token,
                    user_id: user_record.id,
                }));
            }
        }

        log::warn!("sign_in request failed");
        Err(Status::internal("Failed to sing in"))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        log::info!("Handling get_user request");

        let get_user_request = request.into_inner();
        let user_record = self.get_user_by_id(&get_user_request.user_id)?;

        log::info!("get_user request handled successfully");

        Ok(Response::new(UserResponse {
            user_id: user_record.id,
            email: user_record.email,
            first_name: user_record.first_name,
            last_name: user_record.last_name,
        }))
    }

    async fn link_social_account(
        &self,
        request: Request<LinkSocialAccountRequest>,
    ) -> Result<Response<()>, Status> {
        log::info!("Handling link_social_account request");

        let link_social_account_request = request.into_inner();
        self.link_social_account(link_social_account_request)?;

        log::info!("link_social_account request handled successfully");
        Ok(Response::new(()))
    }
}
