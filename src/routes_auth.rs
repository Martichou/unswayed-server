use super::models::{AccessToken, NewAccessToken, NewUser, User};
use super::schema::access_tokens::dsl::*;
use super::schema::users::dsl::*;
use super::Pool;

use crate::diesel::BoolExpressionMethods;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::utils::errors::AppError;
use crate::utils::errors::AppErrorType;

use actix_web::{web, HttpResponse};
use chrono::Duration;
use diesel::dsl::{exists, insert_into, select};
use nanoid::nanoid;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub email: String,
    pub passwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputRefreshToken {
    pub refresh_token: String,
}

fn email_valid(input_email: &str) -> bool {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .expect("Email regex is invalid !");
    email_regex.is_match(input_email)
}

pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, AppError> {
    Ok(web::block(move || add_single_user(db, item))
        .await
        .map(|user| HttpResponse::Created().json(user))?)
}

fn add_single_user(db: web::Data<Pool>, item: web::Json<InputUser>) -> Result<User, AppError> {
    let conn = db.get()?;
    if !email_valid(&item.email) {
        return Err(AppError {
            message: Some("The provided email is not correctly formatted".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        });
    }
    let new_user = NewUser {
        email: &item.email,
        passwd: &item.passwd,
        created_at: chrono::Local::now().naive_local(),
    };
    let item_exist = select(exists(users.filter(email.eq(&item.email)))).get_result(&conn)?;
    if item_exist {
        Err(AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::KeyAlreadyExists,
        })
    } else {
        Ok(insert_into(users).values(&new_user).get_result(&conn)?)
    }
}

pub async fn auth_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, AppError> {
    Ok(web::block(move || auth_single_user(db, item))
        .await
        .map(|access| HttpResponse::Created().json(access))?)
}

fn auth_single_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<AccessToken, AppError> {
    let conn = db.get()?;
    if !email_valid(&item.email) {
        return Err(AppError {
            message: Some("The provided email is not correctly formatted".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        });
    }
    let user_id_f = users
        .filter(email.eq(&item.email).and(passwd.eq(&item.passwd)))
        .select(super::schema::users::dsl::id)
        .first(&conn);
    if user_id_f.is_ok() {
        let new_access_token = NewAccessToken {
            user_id: user_id_f.unwrap(),
            token_type: 1,
            access_token: nanoid!(64),
            refresh_token: nanoid!(64),
            created_at: chrono::Local::now().naive_local(),
            expire_at: chrono::Local::now().naive_local() + Duration::hours(2),
        };
        Ok(insert_into(access_tokens)
            .values(&new_access_token)
            .get_result(&conn)?)
    } else {
        Err(AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::InvalidCrendetials,
        })
    }
}

pub async fn refresh_user(
    db: web::Data<Pool>,
    item: web::Json<InputRefreshToken>,
) -> Result<HttpResponse, AppError> {
    Ok(web::block(move || auth_refresh_token(db, item))
        .await
        .map(|access| HttpResponse::Created().json(access))?)
}

fn auth_refresh_token(
    db: web::Data<Pool>,
    item: web::Json<InputRefreshToken>,
) -> Result<AccessToken, AppError> {
    let conn = db.get()?;
    let access_tokens_f = access_tokens
        .filter(refresh_token.eq(&item.refresh_token))
        .select(user_id)
        .first(&conn);
    if access_tokens_f.is_ok() {
        let new_access_token = NewAccessToken {
            user_id: access_tokens_f.unwrap(),
            token_type: 1,
            access_token: nanoid!(64),
            refresh_token: nanoid!(64),
            created_at: chrono::Local::now().naive_local(),
            expire_at: chrono::Local::now().naive_local() + Duration::hours(2),
        };
        diesel::delete(access_tokens.filter(refresh_token.eq(&item.refresh_token)))
            .execute(&conn)?;
        Ok(insert_into(access_tokens)
            .values(&new_access_token)
            .get_result(&conn)?)
    } else {
        Err(AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::InvalidToken,
        })
    }
}
