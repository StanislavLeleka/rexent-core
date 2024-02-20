// @generated automatically by Diesel CLI.

diesel::table! {
    recommendation_types (reco_type_id) {
        reco_type_id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    user_recommendations (user_reco_id) {
        user_reco_id -> Int4,
        reco_id -> Int4,
        user_id -> Varchar,
    }
}

diesel::joinable!(user_recommendations -> recommendation_types (reco_id));

diesel::allow_tables_to_appear_in_same_query!(
    recommendation_types,
    user_recommendations,
);
