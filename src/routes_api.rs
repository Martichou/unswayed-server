use super::models::Image;
use super::s3_utils::upload::{save_file, split_payload};
use super::schema::images::dsl::*;
use super::schema::users::dsl::*;
use super::Pool;

use crate::diesel::BoolExpressionMethods;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::utils::errors::{AppError, AppErrorType};

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use rusoto_core::Region;
use rusoto_s3::S3;
use rusoto_s3::{GetObjectRequest, S3Client};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoUser {
    pub email: String,
}

fn get_user_id(req: &HttpRequest) -> Option<&str> {
    req.headers().get("user_id")?.to_str().ok()
}

pub async fn get_me(req: HttpRequest, db: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let user_id_f = get_user_id(&req).unwrap().parse::<i32>()?;
    Ok(web::block(move || get_me_info(user_id_f, db))
        .await
        .map(|user| HttpResponse::Ok().json(user))?)
}

fn get_me_info(user_id_f: i32, pool: web::Data<Pool>) -> Result<InfoUser, AppError> {
    let conn = pool.get()?;
    Ok(users
        .filter(super::schema::users::dsl::id.eq(&user_id_f))
        .select(super::schema::users::dsl::email)
        .first::<String>(&conn)
        .map(|emailstd| InfoUser { email: emailstd })?)
}

pub async fn post_upload_one(
    req: HttpRequest,
    mut payload: Multipart,
    db: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    let user_id_f = get_user_id(&req).unwrap().parse::<i32>()?;
    let pl = split_payload(user_id_f, &db, payload.borrow_mut()).await;
    Ok(HttpResponse::Ok().json(save_file(user_id_f, &db, pl).await.unwrap()))
}

pub async fn get_mine(req: HttpRequest, db: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let user_id_f = get_user_id(&req).unwrap().parse::<i32>()?;
    Ok(web::block(move || get_images_list(user_id_f, db))
        .await
        .map(|res| HttpResponse::Ok().json(res))?)
}

fn get_images_list(
    user_id_f: i32,
    pool: web::Data<Pool>,
) -> Result<std::vec::Vec<Image>, AppError> {
    let conn = pool.get()?;
    Ok(images.filter(user_id.eq(user_id_f)).load(&conn)?)
}

pub async fn get_file(req: HttpRequest, db: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let conn = db.get()?;
    let user_id_f = get_user_id(&req).unwrap().parse::<i32>()?;
    let filename = sanitize_filename::sanitize(req.match_info().query("filename"));
    let item_f = images
        .filter(realname.eq(&filename).and(user_id.eq(user_id_f)))
        .select(fakedname)
        .first::<String>(&conn);
    if item_f.is_ok() {
        let s3 = S3Client::new(Region::Custom {
            name: std::env::var("AWS_REGION").unwrap(),
            endpoint: format!(
                "https://s3.{}.scw.cloud",
                std::env::var("AWS_REGION").unwrap()
            ),
        });
        let get_req = GetObjectRequest {
            bucket: std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
            key: item_f.unwrap(),
            ..Default::default()
        };
        let result = s3.get_object(get_req).await?;
        Ok(HttpResponse::Ok().streaming(result.body.unwrap()))
    } else {
        Err(AppError {
            message: Some("Make sure you're the owner of the file you're requesting".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        })
    }
}
