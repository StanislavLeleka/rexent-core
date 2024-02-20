use crate::models::movies::NewLikedMovie;
use crate::utils::db::get_db_conn;
use crate::{models::movies::LikedMovie, schema::liked_movies, DbPool};
use api_error::error::ApiError;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use proto::services::favorites::favorites_service_server::FavoritesService;
use proto::services::favorites::{
    AddLikedMoviesRequest, AddLikedMoviesResponse, GetLikedMoviesRequest, GetLikedMoviesResponse,
};
use tonic::{Request, Response, Status};

pub struct FavoritesServiceImpl {
    db: DbPool,
}

impl FavoritesServiceImpl {
    pub fn new(db: DbPool) -> Self {
        log::info!("Initializing FavoritesService");
        Self { db }
    }

    fn add_liked_movies(&self, request: AddLikedMoviesRequest) -> Result<(), Status> {
        log::info!("Adding liked movies for user_id: {}", request.user_id);

        let conn = &mut get_db_conn(self.db.clone())?;
        conn.transaction::<_, ApiError, _>(|conn| {
            let liked_movies = request
                .movie_id
                .iter()
                .map(|m| NewLikedMovie {
                    user_id: request.user_id.clone(),
                    movie_id: *m,
                })
                .collect::<Vec<_>>();

            diesel::insert_into(liked_movies::table)
                .values(liked_movies)
                .execute(conn)?;

            log::info!(
                "Liked movies added successfully for user_id: {}",
                request.user_id
            );

            Ok(())
        })
        .map_err(|err| {
            log::error!("Error adding liked movies: {:?}", err);
            Status::internal("Failed to add liked movies")
        })
    }

    fn get_liked_movies(&self, user_id: String) -> Result<Vec<LikedMovie>, Status> {
        log::info!("Getting liked movies for user_id: {}", user_id);

        let conn = &mut get_db_conn(self.db.clone())?;
        let liked_movies = liked_movies::table
            .filter(liked_movies::user_id.eq(user_id.clone()))
            .load::<LikedMovie>(conn)
            .map_err(|err| {
                log::error!("Error getting user liked movies: {:?}", err);
                Status::internal("Failed to get user liked movies")
            })?;

        log::info!("Retrieved liked movies by user id: {}", user_id);

        Ok(liked_movies)
    }
}

#[tonic::async_trait]
impl FavoritesService for FavoritesServiceImpl {
    async fn add_liked_movies(
        &self,
        request: Request<AddLikedMoviesRequest>,
    ) -> Result<Response<AddLikedMoviesResponse>, Status> {
        log::info!("Handling add_liked_movies request");

        let add_liked_movies_request = request.into_inner();
        self.add_liked_movies(add_liked_movies_request)?;

        log::info!("sign_up add_liked_movies handled successfully");
        Ok(Response::new(AddLikedMoviesResponse {}))
    }

    async fn get_liked_movies(
        &self,
        request: Request<GetLikedMoviesRequest>,
    ) -> Result<Response<GetLikedMoviesResponse>, Status> {
        log::info!("Handling get_liked_movies request");

        let get_liked_movies_request = request.into_inner();
        let liked_movies = self.get_liked_movies(get_liked_movies_request.user_id)?;
        let movie_ids = liked_movies.iter().map(|m| m.movie_id).collect();

        log::info!("get_liked_movies handled successfully");
        Ok(Response::new(GetLikedMoviesResponse { movie_ids }))
    }
}
