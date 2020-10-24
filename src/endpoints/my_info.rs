use crate::utils::get_user_id::get_user_id;

use crate::schema::users::dsl::*;
use crate::Pool;

use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::utils::errors::AppError;

use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoUser {
    pub email: String,
}

pub async fn my_info(req: HttpRequest, db: web::Data<Pool>) -> Result<HttpResponse, AppError> {
    let user_id_f = get_user_id(&req).unwrap().parse::<i32>()?;
    let conn = db.get()?;
    let res = users
        .filter(id.eq(&user_id_f))
        .select(email)
        .first::<String>(&conn)
        .map(|emailstd| InfoUser { email: emailstd })?;
    Ok(HttpResponse::Ok().json(res))
}
