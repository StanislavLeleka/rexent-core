// @generated automatically by Diesel CLI.

diesel::table! {
    cities (city_id) {
        city_id -> Int4,
        city_name -> Varchar,
        city_code -> Varchar,
        country_id -> Int4,
    }
}

diesel::table! {
    countries (country_id) {
        country_id -> Int4,
        country_code -> Varchar,
        country_name -> Varchar,
    }
}

diesel::table! {
    user_locations (loc_id) {
        loc_id -> Int4,
        lat -> Float8,
        lng -> Float8,
        formatted_address -> Varchar,
        user_id -> Varchar,
        city_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(cities -> countries (country_id));
diesel::joinable!(user_locations -> cities (city_id));

diesel::allow_tables_to_appear_in_same_query!(
    cities,
    countries,
    user_locations,
);
