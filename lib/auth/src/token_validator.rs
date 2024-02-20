use crate::{
    claims::{Claims, ServiceClaims},
    http::auth_middleware::{S2S_ROUTES, UNAUTHORIZED_ROUTES},
    route::Route,
};
use actix_web::{
    dev::ServiceRequest,
    http::{header::HeaderMap, Method},
};
use api_error::error::ApiError;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::de::DeserializeOwned;
use std::env;

pub fn get_user_claims(headers: &HeaderMap) -> Result<Claims, Box<dyn std::error::Error>> {
    let token = extract_token(headers)?;
    let key_env = "JWT_SECRET_KEY";
    let claims: Claims = get_token_claims(&token, key_env)?;
    Ok(claims)
}

pub fn get_s2s_claims(headers: &HeaderMap) -> Result<ServiceClaims, Box<dyn std::error::Error>> {
    let token = extract_token(headers)?;
    let key_env = "JWT_S2S_SECRET_KEY";
    let claims: ServiceClaims = get_token_claims(&token, key_env)?;
    Ok(claims)
}

pub fn extract_token(headers: &HeaderMap) -> Result<String, ApiError> {
    let auth_header = headers
        .get("Authorization")
        .ok_or_else(|| "Authorization header missing")?;

    let auth_token = auth_header
        .to_str()
        .map_err(|_| "Authorization header parsing error")?;

    if !auth_token.starts_with("Bearer ") {
        return Err("Invalid token format".into());
    }

    Ok(auth_token.trim_start_matches("Bearer ").to_string())
}

pub fn is_request_allowed(req: &ServiceRequest) -> bool {
    is_unauthorized_route(req)
        || (is_service_to_service_auth(req) && get_s2s_claims(&req.headers()).is_ok())
        || get_user_claims(&req.headers()).is_ok()
}

fn is_unauthorized_route(req: &ServiceRequest) -> bool {
    let method = req.method();
    *method == Method::OPTIONS
        || check_routes(UNAUTHORIZED_ROUTES.lock().unwrap().get_routes(), req)
}

fn is_service_to_service_auth(req: &ServiceRequest) -> bool {
    let method = req.method();
    *method == Method::OPTIONS || check_routes(S2S_ROUTES.lock().unwrap().get_routes(), req)
}

fn check_routes(routes: &[Route], req: &ServiceRequest) -> bool {
    let method = req.method();
    routes
        .iter()
        .any(|route| route.matches_path_and_method(req.path(), method))
}

pub fn get_token_claims<T: DeserializeOwned>(
    token: &str,
    key_env: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    let key = env::var(key_env)?;
    let key_bytes = DecodingKey::from_base64_secret(&key)?;
    let validation = Validation::new(Algorithm::HS256);

    let claims = decode::<T>(token, &key_bytes, &validation)?;

    Ok(claims.claims)
}
