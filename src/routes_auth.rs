use super::models::{AccessToken, NewAccessToken, NewUser};
use super::schema::access_tokens::dsl::*;
use super::schema::users::dsl::*;
use super::Pool;

use crate::argon2;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::utils::errors::AppError;
use crate::utils::errors::AppErrorType;

use actix_web::{web, HttpResponse};
use argon2::Config;
use chrono::Duration;
use diesel::dsl::{exists, insert_into, select};
use nanoid::nanoid;
use rand::Rng;
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

pub async fn create_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, AppError> {
    let conn = db.get()?;
    if !email_valid(&item.email) {
        return Err(AppError {
            message: Some("The provided email is not correctly formatted".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        });
    }
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    let new_user = NewUser {
        email: &item.email,
        passwd: &argon2::hash_encoded(&item.passwd.as_bytes(), &salt, &config).unwrap(),
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
        insert_into(users).values(&new_user).execute(&conn)?;
        Ok(HttpResponse::Created().finish())
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
        .filter(email.eq(&item.email))
        .select((
            super::schema::users::dsl::id,
            super::schema::users::dsl::passwd,
        ))
        .first::<(i32, String)>(&conn);
    match user_id_f {
        Ok(info) => {
            let matches = argon2::verify_encoded(&info.1, &item.passwd.as_bytes());
            if matches.is_ok() && matches.unwrap() {
                let new_access_token = NewAccessToken {
                    user_id: info.0,
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
        Err(_) => Err(AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::InvalidCrendetials,
        }),
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
    match access_tokens_f {
        Ok(user_idd) => {
            let new_access_token = NewAccessToken {
                user_id: user_idd,
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
        }
        Err(_) => Err(AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::InvalidToken,
        }),
    }
}
