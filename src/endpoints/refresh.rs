use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::{AccessToken, NewAccessToken};
use crate::schema::access_tokens::dsl::*;
use crate::utils::errors::AppError;
use crate::utils::errors::AppErrorType;
use crate::Pool;

use actix_web::{web, HttpResponse};
use chrono::Duration;
use diesel::dsl::insert_into;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputRefreshToken {
    pub refresh_token: String,
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
