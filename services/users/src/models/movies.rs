use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::liked_movies;

#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = liked_movies)]
pub struct LikedMovie {
    pub id: i32,
    pub user_id: String,
    pub movie_id: i32,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = liked_movies)]
pub struct NewLikedMovie {
    pub user_id: String,
    pub movie_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct LikedMovies {
    pub movies: Vec<i32>,
}
