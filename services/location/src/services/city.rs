use diesel::result::Error;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::models::city::{City, CityInput};
use crate::schema::cities;

pub fn get_city(conn: &mut PgConnection, input: &CityInput) -> Result<City, Error> {
    cities::table
        .filter(cities::city_name.eq(&input.city_name))
        .filter(cities::country_id.eq(input.country_id))
        .limit(1)
        .first::<City>(conn)
}

pub fn insert_city(conn: &mut PgConnection, input: &CityInput) -> Result<City, Error> {
    diesel::insert_into(cities::table)
        .values(input)
        .get_result::<City>(conn)
}
