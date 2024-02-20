use crate::{
    models::country::{Country, CountryInput},
    schema::countries,
};
use diesel::{result::Error, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

pub fn get_country(conn: &mut PgConnection, name: &String) -> Result<Country, Error> {
    countries::table
        .filter(countries::country_name.eq(name))
        .limit(1)
        .first::<Country>(conn)
}

pub fn insert_country(conn: &mut PgConnection, input: &CountryInput) -> Result<Country, Error> {
    diesel::insert_into(countries::table)
        .values(input)
        .get_result::<Country>(conn)
}
