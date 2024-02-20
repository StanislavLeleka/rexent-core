// @generated automatically by Diesel CLI.

diesel::table! {
    liked_movies (id) {
        id -> Int4,
        user_id -> Varchar,
        movie_id -> Int4,
    }
}

diesel::table! {
    user_social_accounts (id) {
        id -> Varchar,
        user_id -> Varchar,
        platform -> Varchar,
        account_name -> Varchar,
        access_token -> Varchar,
        refresh_token -> Varchar,
        expires_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        email -> Text,
        pwd_hash -> Text,
        first_name -> Text,
        last_name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(liked_movies -> users (user_id));
diesel::joinable!(user_social_accounts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    liked_movies,
    user_social_accounts,
    users,
);
