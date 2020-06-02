use super::models::User;
use super::schema::users::dsl::*;
use super::Pool;

use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::diesel::QueryDsl;

use actix_web::{web, Error, HttpResponse, HttpRequest};
use futures::{StreamExt, TryStreamExt};
use actix_multipart::Multipart;
use async_std::prelude::*;

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
	mut payload: Multipart
) -> Result<HttpResponse, Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
		let content_type = field.content_disposition().ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type.get_filename().ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filepath = format!("./images/{}", sanitize_filename::sanitize(&filename));
        let mut f = async_std::fs::File::create(filepath).await?;
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}