use chrono::Utc;
use diesel::{ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use tonic::Status;

use crate::{
    clients::geocode::Geocode,
    models::location::{
        Location, LocationResponse, LocationWithCityCountry, NewLocation, UpdateLocation,
    },
    schema::{cities, countries, user_locations},
};

use super::location_service::LocationServiceImpl;

impl LocationServiceImpl {
    pub(super) fn insert(
        &self,
        conn: &mut PgConnection,
        loc: &NewLocation,
        geocode: &Geocode,
        city_id: i32,
        user_id: String,
    ) -> Result<Location, Status> {
        diesel::insert_into(user_locations::table)
            .values((
                user_locations::user_id.eq(user_id),
                user_locations::lat.eq(loc.lat as f64),
                user_locations::lng.eq(loc.lng as f64),
                user_locations::city_id.eq(city_id),
                user_locations::formatted_address.eq(geocode.formatted_address.clone()),
            ))
            .get_result::<Location>(conn)
            .map_err(|err| {
                log::error!("Error inserting new location: {:?}", err);
                Status::internal("Failed to insert location")
            })
    }

    pub(super) fn update(
        &self,
        conn: &mut PgConnection,
        loc: &UpdateLocation,
        geocode: &Geocode,
        city_id: i32,
        user_id: String,
    ) -> Result<(), Status> {
        diesel::update(user_locations::table)
            .filter(user_locations::user_id.eq(user_id))
            .set((
                user_locations::lat.eq(loc.lat as f64),
                user_locations::lng.eq(loc.lng as f64),
                user_locations::city_id.eq(city_id),
                user_locations::formatted_address.eq(geocode.formatted_address.clone()),
                user_locations::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)
            .map_err(|err| {
                log::error!("Error updating location: {:?}", err);
                Status::internal("Failed to update location")
            })?;

        Ok(())
    }

    pub(super) fn get(
        &self,
        conn: &mut PgConnection,
        user_id: String,
    ) -> Result<LocationResponse, Status> {
        let loc = user_locations::table
            .inner_join(cities::table.on(user_locations::city_id.eq(cities::city_id)))
            .inner_join(countries::table.on(cities::country_id.eq(countries::country_id)))
            .filter(user_locations::user_id.eq(user_id))
            .limit(1)
            .first::<LocationWithCityCountry>(conn)
            .map_err(|err| {
                log::error!("Error building location response: {:?}", err);
                Status::internal("Failed to building location")
            })?;

        Ok(loc.into())
    }
}
