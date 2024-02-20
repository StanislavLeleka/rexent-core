use std::env;

use jsonwebtoken::{errors::Error, EncodingKey, Header};
use serde::{Deserialize, Serialize};

static ONE_DAY: i64 = 60 * 60 * 24;

pub fn generate(user_id: String, now: i64) -> Result<String, Error> {
    let claims = Claims::new(user_id, now);
    let key = env::var("JWT_SECRET_KEY").expect("$JWT_SECRET_KEY is not set");
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_base64_secret(&key).unwrap(),
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // aud: String, // Optional. Audience
    exp: i64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: i64, // Optional. Issued at (as UTC timestamp)
    // iss: String, // Optional. Issuer
    // nbf: usize, // Optional. Not Before (as UTC timestamp)
    // sub: String, // Optional. Subject (whom token refers to)
    // ---
    pub user_id: String,
}

impl Claims {
    pub fn new(user_id: String, now: i64) -> Self {
        Claims {
            iat: now,
            exp: now + ONE_DAY,
            user_id,
        }
    }
}
