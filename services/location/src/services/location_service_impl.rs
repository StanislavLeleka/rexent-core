use crate::{
    clients::geocode::{get_address, Geocode},
    models::{
        city::{City, CityInput},
        country::{CountriesResponse, Country, CountryInput},
        location::{LocationResponse, NewLocation, UpdateLocation},
    },
    schema::{cities, countries},
    utils::db::get_db_conn,
};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use proto::services::location::{GetLocationRequest, NewLocationRequest, UpdateLocationRequest};
use tonic::Status;

use super::{
    city::{get_city, insert_city},
    country::{get_country, insert_country},
    location_service::LocationServiceImpl,
};

impl LocationServiceImpl {
    pub(super) async fn create_location(
        &self,
        request: NewLocationRequest,
    ) -> Result<LocationResponse, Status> {
        log::info!(
            "Adding a new location with coordinates (lat: {}, lng: {})",
            request.lat,
            request.lng
        );

        let (_, city, geocode) = self.get_country_and_city(request.lat, request.lng).await?;
        let conn = &mut get_db_conn(self.db.clone())?;

        self.insert(
            conn,
            &NewLocation {
                lat: request.lat,
                lng: request.lng,
            },
            &geocode,
            city.city_id,
            request.user_id.clone(),
        )?;

        log::info!("Location added successfully");

        let response = self.get(conn, request.user_id)?;
        Ok(response.into())
    }

    pub(super) async fn update_location(
        &self,
        request: UpdateLocationRequest,
    ) -> Result<LocationResponse, Status> {
        log::info!(
            "Updating user location with coordinates (lat: {}, lng: {})",
            request.lat,
            request.lng
        );

        let (_, city, geocode) = self.get_country_and_city(request.lat, request.lng).await?;
        let conn = &mut get_db_conn(self.db.clone())?;

        self.update(
            conn,
            &UpdateLocation {
                lat: request.lat,
                lng: request.lng,
            },
            &geocode,
            city.city_id,
            request.user_id.clone(),
        )?;

        log::info!("Location updated successfully");

        let response = self.get(conn, request.user_id)?;
        Ok(response)
    }

    pub(super) fn get_location(
        &self,
        request: GetLocationRequest,
    ) -> Result<LocationResponse, Status> {
        log::info!("Getting location for user: {}", request.user_id);

        let conn = &mut get_db_conn(self.db.clone())?;
        let response = self.get(conn, request.user_id.clone())?;

        log::info!(
            "Successfully retrieved location for user: {}",
            request.user_id
        );
        Ok(response)
    }

    pub(crate) fn get_countries_with_cities(&self) -> Result<CountriesResponse, Status> {
        log::info!("Getting countries and cities list");

        let conn = &mut get_db_conn(self.db.clone())?;
        let countries_with_cities: Vec<(City, Country)> = cities::table
            .inner_join(countries::table)
            .select((City::as_select(), Country::as_select()))
            .load::<(City, Country)>(conn)
            .map_err(|err| {
                log::error!("Error getting countries and cities: {:?}", err);
                Status::internal("Failed to get countries and cities")
            })?;

        log::info!("Successfully retrieved countries and cities list");
        Ok(countries_with_cities.into())
    }

    async fn get_country_and_city(
        &self,
        lat: f32,
        lng: f32,
    ) -> Result<(Country, City, Geocode), Status> {
        let geocode = self
            .geolocation
            .get_geocode(lat, lng)
            .await
            .map_err(|err| {
                log::error!(
                    "Error getting geolocation for (lat: {}, lng: {}): {:?}",
                    lat,
                    lng,
                    err
                );
                Status::internal("Failed to get geolocation from coordinates")
            })?;

        let country_address = get_address(&geocode, "country");
        let city_address = get_address(&geocode, "locality");
        let conn = &mut get_db_conn(self.db.clone())?;

        let country = get_country(conn, &country_address.long_name).unwrap_or_else(|_| {
            insert_country(conn, &CountryInput::from(&country_address)).unwrap()
        });

        let city = get_city(
            conn,
            &CityInput::from_address(&city_address, country.country_id),
        )
        .unwrap_or_else(|_| {
            insert_city(
                conn,
                &CityInput::from_address(&city_address, country.country_id),
            )
            .unwrap()
        });

        Ok((country, city, geocode))
    }
}
