use super::models::{NewUser, User, NewAccessToken, AccessToken};
use super::schema::users::dsl::*;
use super::schema::access_tokens::dsl::{access_tokens, refresh_token, user_id};
use super::Pool;

use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::diesel::QueryDsl;

use actix_web::{web, Error, HttpRequest, HttpResponse};
use diesel::dsl::{insert_into, exists, select};
use serde::{Deserialize, Serialize};
use nanoid::nanoid;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub email: String,
    pub passwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputRefreshToken {
    pub refresh_token: String,
}

fn _get_user_id<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("user_id")?.to_str().ok()
}

pub async fn get_users(
    _req: HttpRequest,
    db: web::Data<Pool>
) -> Result<HttpResponse, Error> {
    // let user_id_f = get_user_id(&req); -> get the user_id of the user asking for the req
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
    if item_exist.is_err() {
        Err(item_exist.unwrap_err())
    } else if item_exist.unwrap() {
        // TODO Create a custom error User already exist
        Err(diesel::result::Error::NotFound)
    } else {
        Ok(insert_into(users).values(&new_user).get_result(&conn)?)
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
    let user_id_f = users.filter(email.eq(&item.email)).select(id).first(&conn);
    if user_id_f.is_ok() {
        let new_access_token = NewAccessToken {
            user_id: user_id_f.unwrap(),
            access_token: nanoid!(64),
            refresh_token: nanoid!(64),
            created_at: chrono::Local::now().naive_local(),
        };
        Ok(insert_into(access_tokens).values(&new_access_token).get_result(&conn)?)
    } else {
        Err(diesel::result::Error::NotFound)
    }
}

pub async fn refresh_user(
    db: web::Data<Pool>,
    item: web::Json<InputRefreshToken>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || auth_refresh_token(db, item))
        .await
        .map(|access| HttpResponse::Created().json(access))
        .map_err(|_| HttpResponse::Unauthorized())?)
}

fn auth_refresh_token(
    db: web::Data<Pool>,
    item: web::Json<InputRefreshToken>,
) -> Result<AccessToken, diesel::result::Error> {
    let conn = db.get().unwrap();

    let access_tokens_f = access_tokens.filter(refresh_token.eq(&item.refresh_token)).select(user_id).first(&conn);
    if access_tokens_f.is_ok() {
        let new_access_token = NewAccessToken {
            user_id: access_tokens_f.unwrap(),
            access_token: nanoid!(64),
            refresh_token: nanoid!(64),
            created_at: chrono::Local::now().naive_local(),
        };
        diesel::delete(access_tokens.filter(refresh_token.eq(&item.refresh_token))).execute(&conn)?;
        Ok(insert_into(access_tokens).values(&new_access_token).get_result(&conn)?)
    } else {
        Err(diesel::result::Error::NotFound)
    }
}