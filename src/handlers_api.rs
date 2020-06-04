use super::s3_utils::upload::{split_payload, save_file};
use super::schema::images::dsl::*;
use super::schema::users::dsl::*;
use super::models::Image;
use super::models::User;
use super::Pool;

use crate::diesel::BoolExpressionMethods;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::diesel::QueryDsl;

use actix_web::{web, Error, HttpResponse, HttpRequest, http::StatusCode};
use rusoto_s3::{GetObjectRequest, S3Client};
use actix_multipart::Multipart;
use std::borrow::BorrowMut;
use rusoto_core::Region;
use rusoto_s3::S3;

fn get_user_id<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("user_id")?.to_str().ok()
}

pub async fn get_me(
    req: HttpRequest,
    db: web::Data<Pool>
) -> Result<HttpResponse, Error> {
    let user_id_f = get_user_id(&req).unwrap().parse::<i32>().unwrap();
    Ok(web::block(move || get_me_info(user_id_f, db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_me_info(
    user_id_f: i32,
    pool: web::Data<Pool>
) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    Ok(users.filter(super::schema::users::dsl::id.eq(&user_id_f)).first::<User>(&conn)?)
}

pub async fn upload_one(
    req: HttpRequest,
    mut payload: Multipart,
    db: web::Data<Pool>
) -> Result<HttpResponse, Error> {
    let user_id_f = get_user_id(&req).unwrap().parse::<i32>().unwrap();
    let pl = split_payload(user_id_f, &db, payload.borrow_mut()).await;
    Ok(HttpResponse::Ok().json(save_file(user_id_f, &db, pl).await.unwrap()))
}

pub async fn get_list(
    req: HttpRequest,
    db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_id_f = get_user_id(&req).unwrap().parse::<i32>().unwrap();
    Ok(web::block(move || get_me_list(user_id_f, db))
        .await
        .map(|res| HttpResponse::Ok().json(res))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_me_list(
    user_id_f: i32,
    pool: web::Data<Pool>,
) -> Result<std::vec::Vec<Image>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    Ok(images.filter(user_id.eq(user_id_f)).load(&conn)?)
}

pub async fn get_one(
    req: HttpRequest,
    db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let conn = db.get().unwrap();
    let user_id_f = get_user_id(&req).unwrap().parse::<i32>().unwrap();
    let filename = sanitize_filename::sanitize(req.match_info().query("filename"));
    let item_f = images.filter(realname.eq(&filename).and(user_id.eq(user_id_f))).select(fakedname).first::<String>(&conn);
    if item_f.is_ok() {
        let s3 = S3Client::new(Region::Custom {
            name: std::env::var("AWS_REGION").unwrap().to_owned(),
            endpoint: format!("https://{}.linodeobjects.com", std::env::var("AWS_REGION").unwrap()).to_owned()
        });
        let get_req = GetObjectRequest {
            bucket: std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
            key: item_f.unwrap(),
            ..Default::default()
        };
        let result = s3.get_object(get_req).await;
        if result.is_ok() {
            Ok(HttpResponse::Ok().streaming(result.unwrap().body.unwrap()))
        } else {
            Ok(HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
    } else {
        Ok(HttpResponse::new(StatusCode::UNAUTHORIZED))
    }
}