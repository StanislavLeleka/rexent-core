use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::user_social_accounts;
use crate::schema::users;

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub email: String,
    pub pwd_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub email: String,
    pub pwd_hash: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = user_social_accounts)]
pub struct NewSocialAccount {
    pub id: String,
    pub user_id: String,
    pub platform: String,
    pub account_name: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: NaiveDateTime,
}
