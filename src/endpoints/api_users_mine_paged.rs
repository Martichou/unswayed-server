use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::Image;
use crate::schema::images::dsl::*;
use crate::utils::errors::{AppError, AppErrorType};
use crate::utils::get_user_id::get_user_id;
use crate::Pool;

use actix_web::{web, HttpRequest, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PagedInfo {
    pub size: i64,
    pub page: i64,
}

pub async fn mine_paged(
    req: HttpRequest,
    db: web::Data<Pool>,
    info: web::Json<PagedInfo>,
) -> Result<HttpResponse, AppError> {
    let user_id_f = get_user_id(&req).unwrap().parse::<i32>()?;
    if info.size > 100 {
        Err(AppError {
            message: Some("The size parameters can't be bigger than 100".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        })
    } else {
        Ok(
            web::block(move || mine_paged_list(user_id_f, db, info.size, info.page))
                .await
                .map(|res| HttpResponse::Ok().json(res))?,
        )
    }
}

fn mine_paged_list(
    user_id_f: i32,
    pool: web::Data<Pool>,
    size: i64,
    page: i64,
) -> Result<std::vec::Vec<Image>, AppError> {
    let conn = pool.get()?;
    Ok(images
        .filter(user_id.eq(user_id_f))
        .limit(size)
        .offset(page * size)
        .load(&conn)?)
}