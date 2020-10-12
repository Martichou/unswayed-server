#[macro_use]
extern crate diesel;
extern crate argon2;

mod endpoints;
mod models;
mod s3_utils;
mod schema;
mod utils;
mod validator;

use actix_web::{web, middleware, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    // Init logging
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

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
        let auth = HttpAuthentication::bearer(validator::validator);
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .route("/auth", web::post().to(endpoints::auth::auth_user))
            .route("/refresh", web::post().to(endpoints::refresh::refresh_user))
            .route("/create", web::post().to(endpoints::create::create_user))
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .service(
                        web::scope("/users")
                            .route("/me", web::get().to(endpoints::api_users_me::me))
                            .route("/mine", web::get().to(endpoints::api_users_mine::mine))
                            .route(
                                "/mine_paged",
                                web::get().to(endpoints::api_users_mine_paged::mine_paged),
                            ),
                    )
                    .service(
                        web::scope("/files")
                            .route(
                                "/upload",
                                web::post().to(endpoints::api_files_upload::upload),
                            )
                            .route(
                                "/get/{filename}",
                                web::get().to(endpoints::api_files_get::get),
                            ),
                    ),
            )
    })
    //.bind(binding)?
    .bind_openssl(binding, builder)?
    .run()
    .await
}
