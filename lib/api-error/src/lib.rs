pub mod error;

#[cfg(test)]
mod tests {
    use crate::error::ApiError;

    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use diesel::result::Error as DieselError;
    use serde_json::json;

    #[test]
    fn test_from_diesel_error_not_found() {
        let err = DieselError::NotFound;
        let api_error = ApiError::from(err);
        assert_eq!(
            api_error,
            ApiError::NotFound(json!({"error": "requested record was not found"}))
        );
    }

    #[test]
    fn test_status_code() {
        let api_error = ApiError::Unauthorized(json!({"error": "Token is invalid"}));
        assert_eq!(api_error.status_code(), StatusCode::UNAUTHORIZED);
    }
}
