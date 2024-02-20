use crate::schema::{recommendation_types, user_recommendations};
use diesel::{
    prelude::{Associations, Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = user_recommendations)]
pub struct NewUserRecommendation {
    pub user_id: String,
    pub reco_id: i32,
}

#[derive(Queryable, Associations, Debug, Clone, Deserialize, Serialize)]
#[diesel(belongs_to(RecommendationType, foreign_key = reco_id))]
#[diesel(table_name = user_recommendations)]
pub struct UserRecommendation {
    pub user_reco_id: i32,
    pub reco_id: i32,
    pub user_id: String,
}

#[derive(Queryable, Debug, Clone, Selectable)]
#[diesel(table_name = recommendation_types)]
pub struct RecommendationType {
    pub reco_type_id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Queryable, Debug, Clone)]
pub struct ExtendedUserRecommendation {
    pub reco_id: i32,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
}
