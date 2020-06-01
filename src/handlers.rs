use super::models::{NewUser, User, NewAccessToken, AccessToken};
use super::schema::users::dsl::*;
use super::schema::access_tokens::dsl::*;
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

fn get_user_id<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("user_id")?.to_str().ok()
}

pub async fn get_me(
    req: HttpRequest,
    db: web::Data<Pool>
) -> Result<HttpResponse, Error> {
    let user_id_f = get_user_id(&req).unwrap().parse::<i32>().unwrap();
    Ok(web::block(move || get_me_info(user_id_f, db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_me_info(
    user_id_f: i32,
    pool: web::Data<Pool>
) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let user = users.filter(super::schema::users::dsl::id.eq(&user_id_f)).first::<User>(&conn)?;
    Ok(user)
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
    let user_id_f = users.filter(email.eq(&item.email)).select(super::schema::users::dsl::id).first(&conn);
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