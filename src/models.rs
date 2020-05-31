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