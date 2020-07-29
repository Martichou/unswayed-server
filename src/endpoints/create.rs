use crate::argon2;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::NewUser;
use crate::schema::users::dsl::*;
use crate::utils::email_valid::email_valid;
use crate::utils::errors::AppError;
use crate::utils::errors::AppErrorType;
use crate::Pool;

use actix_web::{web, HttpResponse};
use argon2::Config;
use diesel::dsl::{exists, insert_into, select};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub email: String,
    pub passwd: String,
}

pub async fn create_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, AppError> {
    let conn = db.get()?;
    if !email_valid(&item.email) {
        return Err(AppError {
            message: Some("The provided email is not correctly formatted".to_string()),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        });
    }
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    let new_user = NewUser {
        email: &item.email,
        passwd: &argon2::hash_encoded(&item.passwd.as_bytes(), &salt, &config).unwrap(),
        created_at: chrono::Local::now().naive_local(),
    };
    let item_exist = select(exists(users.filter(email.eq(&item.email)))).get_result(&conn)?;
    if item_exist {
        Err(AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::KeyAlreadyExists,
        })
    } else {
        insert_into(users).values(&new_user).execute(&conn)?;
        Ok(HttpResponse::Created().finish())
    }
}
