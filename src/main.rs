#[macro_use]
extern crate diesel;

mod routes_auth;
mod routes_api;
mod s3_utils;
mod models;
mod schema;
mod utils;

use actix_web::{dev::ServiceRequest, web, App, Error, http::header, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use utils::errors::{AppError, AppErrorType};
use diesel::r2d2::ConnectionManager;
use schema::access_tokens::dsl::*;
use diesel::prelude::*;
use std::str::FromStr;
use chrono::Duration;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn validate_token(token: &str, pool: web::Data<Pool>) -> Result<(bool, std::string::String), AppError> {
    let conn = pool.get()?;
    let access_token_f = access_tokens.filter(access_token.eq(token)).select((user_id, created_at)).first::<(i32, chrono::NaiveDateTime)>(&conn);
    if access_token_f.is_ok() {
        let access_token_f = access_token_f.unwrap();
        if chrono::Local::now().naive_local() - Duration::hours(2) < access_token_f.1 {
            Ok((true, String::from(access_token_f.0.to_string())))
        } else {
            Err(AppError { message: None, cause: None, error_type: AppErrorType::InvalidToken })
        }
    } else {
        Err(AppError { message: None, cause: None, error_type: AppErrorType::InvalidToken })
    }
}

async fn validator(mut req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);
    let pool = req.app_data::<r2d2::Pool<ConnectionManager<PgConnection>>>();
    match validate_token(credentials.token(), pool.unwrap()) {
        Ok(res) => {
            if res.0 == true {
                req.headers_mut().insert(
                    header::HeaderName::from_str("user_id").unwrap(),
                    header::HeaderValue::from_str(&res.1).unwrap(),
                );
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(res) => Err(res.into()),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let binding = std::env::var("BINDING").expect("BINDING must be set");
    // Checking if envs are set correctly
    std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
    std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
    std::env::var("AWS_S3_BUCKET_NAME").expect("AWS_S3_BUCKET_NAME must be set");
    std::env::var("AWS_REGION").expect("BINDAWS_REGIONING must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .data(pool.clone())
            .route("/auth", web::post().to(routes_auth::auth_user))
            .route("/refresh", web::post().to(routes_auth::refresh_user))
            .route("/users", web::post().to(routes_auth::add_user))
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .route("/me", web::get().to(routes_api::get_me))
                    .route("/upload", web::post().to(routes_api::upload_one))
                    .route("/mine", web::get().to(routes_api::get_mine))
                    .service(
                        web::scope("/get")
                            .route("{filename}", web::get().to(routes_api::get_one))
                    )
            )
    })
    .bind(binding)?
    .run()
    .await
}