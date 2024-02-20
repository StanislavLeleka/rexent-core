use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceClaims {
    pub user_id: String,
}
