use crate::argon2;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::{AccessToken, NewAccessToken};
use crate::schema::access_tokens::dsl::*;
use crate::schema::users::dsl::*;
use crate::utils::email_valid::email_valid;
use crate::utils::errors::AppError;
use crate::utils::errors::AppErrorType;
use crate::Pool;

use actix_web::{web, HttpResponse};
use chrono::Duration;
use diesel::dsl::insert_into;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub email: String,
    pub passwd: String,
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
            crate::schema::users::dsl::id,
            crate::schema::users::dsl::passwd,
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
