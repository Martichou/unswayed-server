use crate::utils::get_user_id::get_user_id;

use crate::schema::images::dsl::*;
use crate::Pool;

use crate::diesel::BoolExpressionMethods;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::utils::errors::{AppError, AppErrorType};

use actix_web::{web, HttpRequest, HttpResponse};
use rusoto_core::Region;
use rusoto_s3::S3;
use rusoto_s3::{GetObjectRequest, S3Client};

pub async fn get(req: HttpRequest, db: web::Data<Pool>) -> Result<HttpResponse, AppError> {
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
