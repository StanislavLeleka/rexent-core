use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub country_code: String,
    pub country_name: String,
    pub city_name: String,
    pub city_code: String,
}
