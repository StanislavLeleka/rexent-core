use crate::{
    models::user::{
        ExtendedUserRecommendation, NewUserRecommendation, RecommendationType, UserRecommendation,
    },
    schema::{recommendation_types, user_recommendations},
};
use diesel::{ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use tonic::Status;

pub fn add_user_recommendation_types(
    conn: &mut PgConnection,
    reco_types: &Vec<NewUserRecommendation>,
) -> Result<(), Status> {
    log::info!("Adding user recommendation types");

    diesel::insert_into(user_recommendations::table)
        .values(reco_types)
        .execute(conn)
        .map_err(|err| {
            log::error!("Error inserting new user recommendation types: {:?}", err);
            Status::internal("Failed to insert user recommendation types")
        })?;

    log::info!("Added recommendation types successfully");

    Ok(())
}

pub fn get_user_recommendation_types(
    conn: &mut PgConnection,
    user_id: String,
) -> Result<Vec<ExtendedUserRecommendation>, Status> {
    log::info!(
        "Fetching user recommendation types for user_id: {}",
        user_id
    );

    let query = user_recommendations::table
        .inner_join(
            recommendation_types::table
                .on(user_recommendations::reco_id.eq(recommendation_types::reco_type_id)),
        )
        .filter(user_recommendations::user_id.eq(&user_id));

    let results: Result<Vec<(UserRecommendation, RecommendationType)>, diesel::result::Error> =
        query.load(conn);

    match results {
        Ok(tuples) => {
            log::info!(
                "Successfully fetched {} recommendation types for user_id: {}",
                tuples.len(),
                user_id
            );
            Ok(tuples
                .into_iter()
                .map(
                    |(user_recommendation, recommendation_type)| ExtendedUserRecommendation {
                        reco_id: user_recommendation.reco_id,
                        user_id: user_recommendation.user_id,
                        name: recommendation_type.name,
                        description: recommendation_type.description,
                    },
                )
                .collect())
        }
        Err(err) => {
            log::error!(
                "Error getting user recommendation types for user_id: {}, err: {:?}",
                user_id,
                err
            );
            Err(Status::internal("Failed to get user recommendation types"))
        }
    }
}

pub fn get_recommendation_types(
    conn: &mut PgConnection,
) -> Result<Vec<RecommendationType>, Status> {
    log::info!("Fetching all recommendation types");

    let results = recommendation_types::table
        .load::<RecommendationType>(conn)
        .map_err(|err| {
            log::error!("Error getting recommendation types: {:?}", err);
            Status::internal("Failed to get recommendation types")
        })?;

    log::info!(
        "Successfully fetched {} recommendation types",
        results.len()
    );

    Ok(results)
}
