use super::models::{NewUser, User, NewAccessToken, AccessToken};
use super::schema::access_tokens::dsl::*;
use super::schema::users::dsl::*;
use super::Pool;

use crate::diesel::BoolExpressionMethods;
use crate::utils::errors::AppErrorType;
use crate::diesel::ExpressionMethods;
use crate::utils::errors::AppError;
use crate::diesel::RunQueryDsl;
use crate::diesel::QueryDsl;

use diesel::dsl::{insert_into, exists, select};
use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse};
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

pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, AppError> {
    Ok(web::block(move || add_single_user(db, item)).await
        .map(|user| HttpResponse::Created().json(user))?)
}

fn add_single_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<User, AppError> {
    let conn = db.get()?;
    let new_user = NewUser { email: &item.email, passwd: &item.passwd, created_at: chrono::Local::now().naive_local() };
    let item_exist = select(exists(users.filter(email.eq(&item.email)))).get_result(&conn)?;
    if item_exist {
        Err(AppError { message: None, cause: None, error_type: AppErrorType::KeyAlreadyExists })
    } else {
        Ok(insert_into(users).values(&new_user).get_result(&conn)?)
    }
}

pub async fn auth_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, AppError> {
    Ok(web::block(move || auth_single_user(db, item)).await
        .map(|access| HttpResponse::Created().json(access))?)
}

fn auth_single_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<AccessToken, AppError> {
    let conn = db.get()?;
    let user_id_f = users.filter(email.eq(&item.email).and(passwd.eq(&item.passwd))).select(super::schema::users::dsl::id).first(&conn);
    if user_id_f.is_ok() {
        let new_access_token = NewAccessToken {
            user_id: user_id_f.unwrap(),
            access_token: nanoid!(64),
            refresh_token: nanoid!(64),
            created_at: chrono::Local::now().naive_local(),
        };
        Ok(insert_into(access_tokens).values(&new_access_token).get_result(&conn)?)
    } else {
        Err(AppError { message: None, cause: None, error_type: AppErrorType::InvalidCrendetials })
    }
}

pub async fn refresh_user(
    db: web::Data<Pool>,
    item: web::Json<InputRefreshToken>,
) -> Result<HttpResponse, AppError> {
    Ok(web::block(move || auth_refresh_token(db, item)).await
        .map(|access| HttpResponse::Created().json(access))?)
}

fn auth_refresh_token(
    db: web::Data<Pool>,
    item: web::Json<InputRefreshToken>,
) -> Result<AccessToken, AppError> {
    let conn = db.get()?;
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
        Err(AppError { message: None, cause: None, error_type: AppErrorType::InvalidToken })
    }
}