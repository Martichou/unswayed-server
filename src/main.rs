#[macro_use]
extern crate diesel;
extern crate argon2;

mod endpoints;
mod models;
mod s3_utils;
mod schema;
mod utils;
mod validator;

use actix_web::{middleware, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use models::Pool;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

fn get_ssl_builder() -> openssl::ssl::SslAcceptorBuilder {
    let key = std::env::var("KEY_PRIV").expect("BINDING must be set");
    let cert = std::env::var("KEY_CERT").expect("BINDING must be set");
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(key, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(cert).unwrap();

    builder
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Load env variable from .env
    dotenv::dotenv().ok();
    // Define the verbose of the logs - info for general and actix
    std::env::set_var("RUST_LOG", "info,actix_server=info,actix_web=info");
    // Init the log module
    env_logger::init();

    std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID must be set");
    std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set");
    std::env::var("AWS_S3_BUCKET_NAME").expect("AWS_S3_BUCKET_NAME must be set");
    std::env::var("AWS_REGION").expect("BINDAWS_REGIONING must be set");

    async_std::fs::create_dir_all("./tmp").await?;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator::validator);
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .route(
                "/auth_user",
                web::post().to(endpoints::auth_user::auth_user),
            )
            .route(
                "/refresh_token",
                web::post().to(endpoints::refresh_token::refresh_token_user),
            )
            .route(
                "/create_user",
                web::post().to(endpoints::create_user::create_user),
            )
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .service(
                        web::scope("/users")
                            .route("/me", web::get().to(endpoints::my_info::my_info))
                            .route("/my_files", web::get().to(endpoints::my_files::my_files)),
                    )
                    .service(
                        web::scope("/files")
                            .route(
                                "/upload",
                                web::post().to(endpoints::upload_files::upload_files),
                            )
                            .route(
                                "/get/{filename}",
                                web::get().to(endpoints::get_file::get_file),
                            ),
                    ),
            )
    })
    .bind_openssl(
        std::env::var("BINDING").expect("Missing binding"),
        get_ssl_builder(),
    )?
    .run()
    .await
}
