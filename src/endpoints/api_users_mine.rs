use crate::utils::get_user_id::get_user_id;

use crate::models::Image;
use crate::schema::images::dsl::*;
use crate::Pool;

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::utils::errors::AppError;

use actix_web::{web, HttpRequest, HttpResponse};

pub async fn mine(req: HttpRequest, db: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let user_id_f = get_user_id(&req).unwrap().parse::<i32>()?;
    Ok(web::block(move || mine_list(user_id_f, db))
        .await
        .map(|res| HttpResponse::Ok().json(res))?)
}

fn mine_list(user_id_f: i32, pool: web::Data<Pool>) -> Result<std::vec::Vec<Image>, AppError> {
    let conn = pool.get()?;
    Ok(images.filter(user_id.eq(user_id_f)).load(&conn)?)
}
