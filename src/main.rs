#[macro_use]
extern crate diesel;
extern crate argon2;

mod models;
mod routes_api;
mod routes_auth;
mod s3_utils;
mod schema;
mod utils;

use actix_web::{dev::ServiceRequest, http::header, web, App, Error, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use chrono::Duration;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use schema::access_tokens::dsl::*;
use std::str::FromStr;
use utils::errors::{AppError, AppErrorType};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn validate_token(
    token: &str,
    pool: web::Data<Pool>,
) -> Result<(bool, std::string::String, i32, &str), AppError> {
    let conn = pool.get()?;
    let access_token_f = access_tokens
        .filter(access_token.eq(token))
        .select((user_id, expire_at, token_type))
        .first::<(i32, chrono::NaiveDateTime, i32)>(&conn);
    match access_token_f {
        Ok(info) => {
            if chrono::Local::now().naive_local() - Duration::hours(2) < info.1 {
                Ok((true, info.0.to_string(), info.2, token))
            } else {
                Err(AppError {
                    message: None,
                    cause: None,
                    error_type: AppErrorType::InvalidToken,
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

async fn validator(
    mut req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);
    let pool = req.app_data::<r2d2::Pool<ConnectionManager<PgConnection>>>();
    match validate_token(credentials.token(), pool.unwrap()) {
        Ok(res) => {
            if res.0 {
                req.headers_mut().insert(
                    header::HeaderName::from_str("user_id").unwrap(),
                    header::HeaderValue::from_str(&res.1).unwrap(),
                );
                req.headers_mut().insert(
                    header::HeaderName::from_str("token_type").unwrap(),
                    header::HeaderValue::from_str(&res.2.to_string()).unwrap(),
                );
                if res.2 == 2 {
                    req.headers_mut().insert(
                        header::HeaderName::from_str("token").unwrap(),
                        header::HeaderValue::from_str(&res.3).unwrap(),
                    );
                }
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
    let key = std::env::var("KEY_PRIV").expect("BINDING must be set");
    let cert = std::env::var("KEY_CERT").expect("BINDING must be set");
    // Checking if envs are set correctly
    std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
    std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
    std::env::var("AWS_S3_BUCKET_NAME").expect("AWS_S3_BUCKET_NAME must be set");
    std::env::var("AWS_REGION").expect("BINDAWS_REGIONING must be set");

    async_std::fs::create_dir_all("./tmp").await?;

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(key, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(cert).unwrap();

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

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
                    .service(
                        web::scope("/users")
                            .route("/me", web::get().to(routes_api::get_me))
                            .route("/mine", web::get().to(routes_api::get_mine)),
                    )
                    .service(
                        web::scope("/special")
                            .route("token", web::get().to(routes_api::get_specialtoken))
                            .route(
                                "token_keepalive",
                                web::patch().to(routes_api::patch_specialtoken_keepalive),
                            ),
                    )
                    .service(
                        web::scope("/files")
                            .route("/upload", web::post().to(routes_api::post_upload_one))
                            .route("/get/{filename}", web::get().to(routes_api::get_file)),
                    ),
            )
    })
    .bind(binding)?
    //.bind_openssl(binding, builder)?
    .run()
    .await
}
