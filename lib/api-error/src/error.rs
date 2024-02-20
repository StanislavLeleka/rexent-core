use actix_web::{http::StatusCode, HttpResponse};
use bcrypt::BcryptError;
use diesel::r2d2::{Error as R2D2Error, PoolError};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use serde_json::{json, Value as JsonValue};
use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug, PartialEq)]
pub enum ApiError {
    #[error("Unauthorized: {}", _0)]
    Unauthorized(JsonValue),

    #[error("Forbidden: {}", _0)]
    Forbidden(JsonValue),

    #[error("Not Found: {}", _0)]
    NotFound(JsonValue),

    #[error("Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),

    #[error("Internal Server Error: {}", _0)]
    InternalServerError(JsonValue),

    //add another possible http errors here
    #[error("Bad Request: {}", _0)]
    BadRequest(JsonValue),
}

impl From<DieselError> for ApiError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    ApiError::UnprocessableEntity(json!({ "error": message }))
                } else {
                    ApiError::InternalServerError(json!({"error": info.message()}))
                }
            }
            DieselError::NotFound => {
                ApiError::NotFound(json!({"error": "requested record was not found"}))
            }
            _ => ApiError::InternalServerError(json!({"error": "Internal Server Error"})),
        }
    }
}

impl actix_web::error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::Unauthorized(ref msg) => HttpResponse::Unauthorized().json(msg),
            ApiError::Forbidden(ref msg) => HttpResponse::Forbidden().json(msg),
            ApiError::NotFound(ref msg) => HttpResponse::NotFound().json(msg),
            ApiError::UnprocessableEntity(ref msg) => HttpResponse::UnprocessableEntity().json(msg),
            ApiError::InternalServerError(ref msg) => HttpResponse::InternalServerError().json(msg),
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
        }
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<R2D2Error> for ApiError {
    fn from(value: R2D2Error) -> Self {
        ApiError::InternalServerError(json!({"error": "Internal Server Error"}))
    }
}

impl From<PoolError> for ApiError {
    fn from(_err: PoolError) -> Self {
        ApiError::InternalServerError(json!({"error": "Internal Server Error"}))
    }
}

impl From<BcryptError> for ApiError {
    fn from(_err: BcryptError) -> Self {
        ApiError::InternalServerError(json!({"error": "Internal Server Error"}))
    }
}

impl From<ApiError> for HttpResponse {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::InternalServerError(_) => HttpResponse::InternalServerError().finish(),
            ApiError::UnprocessableEntity(message) => {
                HttpResponse::BadRequest().body(message.to_string())
            }
            // Add more cases for other error variants if needed
            _ => todo!(),
        }
    }
}

impl From<JwtError> for ApiError {
    fn from(err: JwtError) -> Self {
        match err.kind() {
            JwtErrorKind::InvalidToken => ApiError::Unauthorized(json!({
                "error": "Token is invalid"
            })),
            JwtErrorKind::InvalidIssuer => ApiError::Unauthorized(json!({
                "error": "Issuer is invalid",
            })),
            _ => ApiError::Unauthorized(json!({
                "error": "An issue was found with the token provided",
            })),
        }
    }
}

impl From<&str> for ApiError {
    fn from(value: &str) -> Self {
        ApiError::InternalServerError(json!({ "error": value }))
    }
}

impl From<Status> for ApiError {
    fn from(value: Status) -> Self {
        match value.code() {
            tonic::Code::Ok => todo!(),
            tonic::Code::Cancelled => todo!(),
            tonic::Code::Unknown => todo!(),
            tonic::Code::InvalidArgument => {
                ApiError::UnprocessableEntity(json!({ "error": value.message() }))
            }
            tonic::Code::DeadlineExceeded => {
                ApiError::InternalServerError(json!({ "error": value.message() }))
            }
            tonic::Code::NotFound => ApiError::NotFound(json!({ "error": value.message() })),
            tonic::Code::AlreadyExists => {
                ApiError::UnprocessableEntity(json!({ "error": value.message() }))
            }
            tonic::Code::PermissionDenied => {
                ApiError::Unauthorized(json!({ "error": value.message() }))
            }
            tonic::Code::ResourceExhausted => todo!(),
            tonic::Code::FailedPrecondition => todo!(),
            tonic::Code::Aborted => {
                ApiError::InternalServerError(json!({ "error": value.message() }))
            }
            tonic::Code::OutOfRange => todo!(),
            tonic::Code::Unimplemented => todo!(),
            tonic::Code::Internal => {
                ApiError::InternalServerError(json!({ "error": value.message() }))
            }
            tonic::Code::Unavailable => todo!(),
            tonic::Code::DataLoss => todo!(),
            tonic::Code::Unauthenticated => {
                ApiError::Unauthorized(json!({ "error": value.message() }))
            }
        }
    }
}
