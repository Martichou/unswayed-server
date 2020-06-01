#[macro_use]
extern crate diesel;

use crate::errors::ServiceError;
use schema::access_tokens::dsl::*;

use actix_web::{dev::ServiceRequest, web, App, Error, http::header, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::str::FromStr;

mod handlers;
mod errors;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn validate_token(token: &str, pool: web::Data<Pool>) -> Result<(bool, std::string::String), ServiceError> {
    let conn = pool.get().unwrap();
    // TODO Implement the expired token
    let user_id_f: std::result::Result<i32, diesel::result::Error> = access_tokens.filter(access_token.eq(token)).select(user_id).first(&conn);
    if user_id_f.is_ok() {
        // TODO pass user_id to the req
        Ok((true, std::string::String::from(user_id_f.unwrap().to_string())))
    } else {
        Ok((false, std::string::String::from("no_user_id")))
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
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .data(pool.clone())
            .route("/auth", web::post().to(handlers::auth_user))
            .route("/users", web::post().to(handlers::add_user))
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .route("/users", web::get().to(handlers::get_users))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}