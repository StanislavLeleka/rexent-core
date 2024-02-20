use actix_web::http::header::HeaderMap;
use api_error::error::ApiError;
use serde_json::json;

use crate::token_validator::get_user_claims;

pub fn get_user_id(headers: &HeaderMap) -> Result<String, ApiError> {
    if let Ok(claims) = get_user_claims(headers) {
        Ok(claims.user_id)
    } else {
        Err(ApiError::Unauthorized(json!({"error": "Invalid token"})))
    }
}
