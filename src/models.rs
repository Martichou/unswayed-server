use crate::schema::*;

use diesel::*;
use serde::{Deserialize, Serialize};

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
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "access_tokens"]
pub struct NewAccessToken<'a> {
	pub user_id: &'a i32,
    pub access_token: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Queryable, Identifiable)]
pub struct RefreshToken {
    pub id: i32,
	pub user_id: i32,
	pub refresh_token: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "refresh_tokens"]
pub struct NewRefreshToken<'a> {
	pub user_id: &'a i32,
    pub refresh_token: &'a str,
    pub created_at: chrono::NaiveDateTime,
}