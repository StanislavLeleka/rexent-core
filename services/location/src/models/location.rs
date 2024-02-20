use super::city::City;
use super::country::Country;
use crate::schema::user_locations;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Associations, Debug, Clone, Deserialize, Serialize)]
#[diesel(belongs_to(City, foreign_key = city_id))]
#[diesel(table_name = user_locations)]
pub struct Location {
    pub loc_id: i32,
    pub lat: f64,
    pub lng: f64,
    pub formatted_address: String,
    pub user_id: String,
    pub city_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = user_locations)]
pub struct CreateLocation {
    pub lat: f64,
    pub lng: f64,
    pub formatted_address: String,
    pub user_id: String,
    pub city_id: i32,
}

#[derive(Queryable, Debug, Clone)]
pub struct LocationWithCityCountry {
    pub location: Location,
    pub city: City,
    pub country: Country,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct NewLocation {
    pub lat: f32,
    pub lng: f32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct UpdateLocation {
    pub lat: f32,
    pub lng: f32,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct LocationResponse {
    pub lat: f64,
    pub lng: f64,
    pub country_code: String,
    pub country_name: String,
    pub city_name: String,
    pub city_code: String,
    pub formatted_address: String,
    pub user_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<LocationWithCityCountry> for LocationResponse {
    fn from(value: LocationWithCityCountry) -> Self {
        LocationResponse {
            lat: value.location.lat,
            lng: value.location.lng,
            country_code: value.country.country_code,
            country_name: value.country.country_name,
            city_name: value.city.city_name,
            city_code: value.city.city_code,
            formatted_address: value.location.formatted_address,
            user_id: value.location.user_id,
            created_at: value.location.created_at,
            updated_at: value.location.updated_at,
        }
    }
}

impl From<LocationResponse> for proto::services::location::LocationResponse {
    fn from(value: LocationResponse) -> Self {
        proto::services::location::LocationResponse {
            location: Some(proto::services::location::Location {
                lat: value.lat as f32,
                lng: value.lng as f32,
                country_code: Some(value.country_code),
                country_name: Some(value.country_name),
                city_name: Some(value.city_name),
                city_code: Some(value.city_code),
                formatted_address: Some(value.formatted_address),
            }),
            user_id: value.user_id,
        }
    }
}
