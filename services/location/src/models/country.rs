use super::city::{City, CityResponse};
use crate::{clients::geocode::LocationInfo, schema::countries};
use diesel::{Insertable, Queryable, Selectable};
use std::collections::HashMap;

#[derive(Queryable, Debug, Clone, Selectable)]
#[diesel(table_name = countries)]
pub struct Country {
    pub country_id: i32,
    pub country_code: String,
    pub country_name: String,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = countries)]
pub struct CountryInput {
    pub country_code: String,
    pub country_name: String,
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

impl From<Vec<(City, Country)>> for CountriesResponse {
    fn from(value: Vec<(City, Country)>) -> Self {
        let mut countries: HashMap<(String, String), Vec<CityResponse>> = HashMap::new();

        for (city, country) in value {
            let city_response = countries
                .entry((country.country_code.clone(), country.country_name.clone()))
                .or_insert_with(Vec::new);

            city_response.push(CityResponse {
                city_name: city.city_name.clone(),
                city_code: city.city_code.clone(),
            });
        }

        let countries = countries
            .into_iter()
            .map(|((country_code, country_name), cities)| CountryResponse {
                country_name,
                country_code,
                cities,
            })
            .collect::<Vec<CountryResponse>>();

        CountriesResponse { countries }
    }
}

impl From<&LocationInfo> for CountryInput {
    fn from(address: &LocationInfo) -> Self {
        Self {
            country_name: address.long_name.clone(),
            country_code: address.short_name.clone(),
        }
    }
}

impl From<CountriesResponse> for proto::services::location::CountriesResponse {
    fn from(value: CountriesResponse) -> Self {
        proto::services::location::CountriesResponse {
            countries: value
                .countries
                .iter()
                .map(|country| proto::services::location::CountryResponse {
                    country_code: country.country_code.clone(),
                    country_name: country.country_name.clone(),
                    cities: country
                        .cities
                        .iter()
                        .map(|city| proto::services::location::CityResponse {
                            city_name: city.city_name.clone(),
                            city_code: city.city_code.clone(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}
