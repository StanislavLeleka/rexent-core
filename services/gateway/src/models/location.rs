use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct NewLocation {
    pub lat: f32,
    pub lng: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateLocation {
    pub lat: f32,
    pub lng: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LocationResponse {
    pub lat: f32,
    pub lng: f32,
    pub country_code: String,
    pub country_name: String,
    pub city_name: String,
    pub city_code: String,
    pub formatted_address: String,
    pub user_id: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct CityResponse {
    pub city_name: String,
    pub city_code: String,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct CountryResponse {
    pub country_code: String,
    pub country_name: String,
    pub cities: Vec<CityResponse>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct CountriesResponse {
    pub countries: Vec<CountryResponse>,
}

impl NewLocation {
    pub fn to_grpc(&self, user_id: String) -> proto::services::location::NewLocationRequest {
        proto::services::location::NewLocationRequest {
            user_id,
            lat: self.lat,
            lng: self.lng,
        }
    }
}

impl UpdateLocation {
    pub fn to_grpc(&self, user_id: String) -> proto::services::location::UpdateLocationRequest {
        proto::services::location::UpdateLocationRequest {
            user_id,
            lat: self.lat,
            lng: self.lng,
        }
    }
}

impl From<proto::services::location::LocationResponse> for LocationResponse {
    fn from(value: proto::services::location::LocationResponse) -> Self {
        let location = value.location.unwrap();

        LocationResponse {
            lat: location.lat,
            lng: location.lng,
            country_code: location.country_code.unwrap(),
            country_name: location.country_name.unwrap(),
            city_name: location.city_name.unwrap(),
            city_code: location.city_code.unwrap(),
            formatted_address: location.formatted_address.unwrap(),
            user_id: value.user_id,
        }
    }
}

impl From<proto::services::location::CountriesResponse> for CountriesResponse {
    fn from(value: proto::services::location::CountriesResponse) -> Self {
        CountriesResponse {
            countries: value
                .countries
                .iter()
                .map(|country| CountryResponse {
                    country_code: country.country_code.clone(),
                    country_name: country.country_name.clone(),
                    cities: country
                        .cities
                        .iter()
                        .map(|city| CityResponse {
                            city_name: city.city_name.clone(),
                            city_code: city.city_code.clone(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}
