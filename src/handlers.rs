use super::models::{NewUser, User, NewAccessToken, AccessToken};
use super::schema::users::dsl::*;
use super::schema::access_tokens::dsl::{access_tokens};
use super::Pool;

use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::diesel::QueryDsl;

use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{insert_into, exists, select};
use serde::{Deserialize, Serialize};
use nanoid::nanoid;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub email: String,
    pub passwd: String,
}

pub async fn get_users(
    db: web::Data<Pool>
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_users(
    pool: web::Data<Pool>
) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = users.load::<User>(&conn)?;
    Ok(items)
}

pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_user(db, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::Conflict())?)
}

fn add_single_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        email: &item.email,
        passwd: &item.passwd,
        created_at: chrono::Local::now().naive_local(),
    };
    let item_exist: std::result::Result<bool, diesel::result::Error> = select(exists(users.filter(email.eq(&item.email)))).get_result(&conn);
    if item_exist.is_err() || item_exist.unwrap() {
        // TODO Create a custom error
        Err(diesel::result::Error::NotFound)
    } else {
        let res = insert_into(users).values(&new_user).get_result(&conn)?;
        Ok(res)
    }
}

pub async fn auth_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || auth_single_user(db, item))
        .await
        .map(|access| HttpResponse::Created().json(access))
        .map_err(|_| HttpResponse::Unauthorized())?)
}

fn auth_single_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<AccessToken, diesel::result::Error> {
    let conn = db.get().unwrap();
    let user_id = users.filter(email.eq(&item.email)).select(id).first(&conn);  
    if user_id.is_ok() {
        // TODO Check if an already existing token is present for that user
        // If we return that one
        // Else return a new one
        let new_access_token = NewAccessToken {
            user_id: user_id.unwrap(),
            access_token: nanoid!(64),
            refresh_token: nanoid!(64),
            created_at: chrono::Local::now().naive_local(),
        };
        let res = insert_into(access_tokens).values(&new_access_token).get_result(&conn)?;
        Ok(res)
    } else {
        Err(diesel::result::Error::NotFound)
    }
}