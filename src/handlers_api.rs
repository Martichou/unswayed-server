use super::s3_utils::upload::{split_payload, save_file};
use super::schema::users::dsl::*;
use super::models::User;
use super::Pool;

use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::diesel::QueryDsl;

use actix_web::{web, Error, HttpResponse, HttpRequest};
use actix_multipart::Multipart;
use std::borrow::BorrowMut;

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
    let user = users.filter(super::schema::users::dsl::id.eq(&user_id_f)).first::<User>(&conn)?;
    Ok(user)
}

pub async fn upload_one(
    req: HttpRequest,
    mut payload: Multipart,
    db: web::Data<Pool>
) -> Result<HttpResponse, Error> {
    let user_id_f = get_user_id(&req).unwrap().parse::<i32>().unwrap();
    let pl = split_payload(user_id_f, &db, payload.borrow_mut()).await;
    let callback = save_file(user_id_f, &db, pl).await.unwrap();
    Ok(HttpResponse::Ok().json(callback))
}