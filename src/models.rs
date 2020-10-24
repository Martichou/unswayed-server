use crate::schema::*;

use diesel::r2d2::ConnectionManager;
use diesel::*;
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub passwd: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub passwd: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct AccessToken {
    pub id: i32,
    pub user_id: i32,
    pub access_token: String,
    pub refresh_token: String,
    pub created_at: chrono::NaiveDateTime,
    pub expire_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize)]
#[table_name = "access_tokens"]
pub struct NewAccessToken {
    pub user_id: i32,
    pub access_token: std::string::String,
    pub refresh_token: std::string::String,
    pub created_at: chrono::NaiveDateTime,
    pub expire_at: chrono::NaiveDateTime,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "ufile"]
pub struct UFile {
    pub id: i32,
    pub user_id: i32,
    pub realname: std::string::String,
    pub fakedname: std::string::String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "ufile"]
pub struct NewUFile {
    pub user_id: i32,
    pub realname: std::string::String,
    pub fakedname: std::string::String,
    pub created_at: chrono::NaiveDateTime,
}
