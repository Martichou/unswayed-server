use crate::utils::get_user_id::get_user_id;

use crate::s3_utils::upload::{save_file, split_payload};
use crate::Pool;

use crate::utils::errors::AppError;

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use std::borrow::BorrowMut;

pub async fn upload_files(
    req: HttpRequest,
    mut payload: Multipart,
    db: web::Data<Pool>,
) -> Result<HttpResponse, AppError> {
    let user_id_f = get_user_id(&req).unwrap().parse::<i32>()?;
    let pl = split_payload(user_id_f, &db, payload.borrow_mut()).await;
    Ok(HttpResponse::Ok().json(save_file(user_id_f, &db, pl).await.unwrap()))
}
