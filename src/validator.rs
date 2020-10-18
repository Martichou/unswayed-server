use crate::schema::access_tokens::dsl::*;
use crate::utils::errors::AppError;
use crate::utils::errors::AppErrorType;
use crate::Pool;

use actix_web::{dev::ServiceRequest, http::header, Error};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use chrono::Duration;
use diesel::prelude::*;

fn validate_token(token: &str, pool: &Pool) -> Result<(bool, std::string::String), AppError> {
    let conn = pool.get()?;
    let access_token_f = access_tokens
        .filter(access_token.eq(token))
        .select((user_id, expire_at))
        .first::<(i32, chrono::NaiveDateTime)>(&conn);
    match access_token_f {
        Ok(info) => {
            if chrono::Local::now().naive_local() - Duration::hours(2) < info.1 {
                Ok((true, info.0.to_string()))
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
            error_type: AppErrorType::InvalidToken,
        }),
    }
}

pub async fn validator(
    mut req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    match validate_token(credentials.token(), req.app_data::<Pool>().unwrap()) {
        Ok(res) => {
            if res.0 {
                req.headers_mut().insert(
                    header::HeaderName::from_static("user_id"),
                    header::HeaderValue::from_str(&res.1).unwrap(),
                );
                Ok(req)
            } else {
                let config = req
                    .app_data::<Config>()
                    .cloned()
                    .unwrap_or_else(Default::default);
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(res) => Err(res.into()),
    }
}
