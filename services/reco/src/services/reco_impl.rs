use crate::{
    models::{mappers::Mapper, user::NewUserRecommendation},
    recommender::movie::MovieRecommender,
    services::user_reco_crud::{
        add_user_recommendation_types, get_recommendation_types, get_user_recommendation_types,
    },
    utils::db::get_db_conn,
};
use db::postgres::DbPool;
use proto::services::reco::{
    reco_service_server::RecoService, AddUserRecommendationTypesRequest,
    AddUserRecommendationTypesResponse, GetMovieRecommendationsRequest,
    GetMovieRecommendationsResponse, GetRecommendationTypesRequest, GetRecommendationTypesResponse,
    GetUserRecommendationTypesRequest, GetUserRecommendationTypesResponse,
};
use tonic::{Request, Response, Status};

pub struct RecoServiceImpl {
    pub movie_recommender: MovieRecommender,
    pub db: DbPool,
}

impl RecoServiceImpl {
    pub fn new(movie_recommender: MovieRecommender, db: DbPool) -> Self {
        Self {
            movie_recommender,
            db,
        }
    }
}

#[tonic::async_trait]
impl RecoService for RecoServiceImpl {
    async fn get_movie_recommendations(
        &self,
        request: Request<GetMovieRecommendationsRequest>,
    ) -> Result<Response<GetMovieRecommendationsResponse>, Status> {
        log::info!("Handling get_movie_recommendations request");

        let request = request.into_inner();
        let recommendations = self
            .movie_recommender
            .get_recommendations(request.user_id)
            .await?;

        log::info!("get_movie_recommendations request handled successfully");

        Ok(Response::new(Mapper::map_to_movie_reco_response(
            recommendations,
        )))
    }

    async fn add_user_recommendation_types(
        &self,
        request: Request<AddUserRecommendationTypesRequest>,
    ) -> Result<Response<AddUserRecommendationTypesResponse>, Status> {
        log::info!("Handling add_user_recommendation_types request");

        let request = request.into_inner();
        let user_id = &request.user_id;

        let recommendations: Vec<NewUserRecommendation> = request
            .reco_ids
            .iter()
            .map(|r| NewUserRecommendation {
                user_id: user_id.clone(),
                reco_id: r.clone(),
            })
            .collect();

        let conn = &mut get_db_conn(self.db.clone())?;
        add_user_recommendation_types(conn, &recommendations)?;

        log::info!("add_user_recommendation_types request handled successfully");

        Ok(Response::new(AddUserRecommendationTypesResponse {
            success: true,
        }))
    }

    async fn get_user_recommendation_types(
        &self,
        request: Request<GetUserRecommendationTypesRequest>,
    ) -> Result<Response<GetUserRecommendationTypesResponse>, Status> {
        log::info!("Handling get_user_recommendation_types request");

        let request = request.into_inner();
        let conn = &mut get_db_conn(self.db.clone())?;
        let user_recommendations = get_user_recommendation_types(conn, request.user_id.clone())?;

        log::info!("get_user_recommendation_types request handled successfully");

        Ok(Response::new(Mapper::map_to_user_reco_response(
            user_recommendations,
        )))
    }

    async fn get_recommendation_types(
        &self,
        _request: Request<GetRecommendationTypesRequest>,
    ) -> Result<Response<GetRecommendationTypesResponse>, Status> {
        log::info!("Handling get_recommendation_types request");

        let conn = &mut get_db_conn(self.db.clone())?;
        let recommendation_types = get_recommendation_types(conn)?;

        log::info!("get_recommendation_types request handled successfully");

        Ok(Response::new(Mapper::map_to_reco_type_response(
            recommendation_types,
        )))
    }
}
