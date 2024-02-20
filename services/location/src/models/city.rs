use super::country::Country;
use crate::{clients::geocode::LocationInfo, schema::cities};
use diesel::{Associations, Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Associations, Debug, Clone, Selectable)]
#[diesel(belongs_to(Country, foreign_key = country_id))]
#[diesel(table_name = cities)]
pub struct City {
    pub city_id: i32,
    pub city_name: String,
    pub city_code: String,
    pub country_id: i32,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = cities)]
pub struct CityInput {
    pub city_name: String,
    pub city_code: String,
    pub country_id: i32,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct CityResponse {
    pub city_name: String,
    pub city_code: String,
}

#[derive(Serialize)]
pub struct NewLocationSQSMessage {
    pub country_code: String,
    pub country_name: String,
    pub city_name: String,
    pub city_code: String,
}

impl CityInput {
    pub fn from_address(address: &LocationInfo, country_id: i32) -> Self {
        Self {
            city_name: address.long_name.clone(),
            city_code: address.short_name.clone(),
            country_id,
        }
    }
}
