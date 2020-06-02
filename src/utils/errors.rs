use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
	InternalServerError,
    #[display(fmt = "BadRequest: {}", _0)]
	BadRequest(String),
	#[display(fmt = "Invalid Token")]
	InvalidToken,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError().json("Internal Server Error, Please try later"),
			ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
			ServiceError::InvalidToken => HttpResponse::BadRequest().json("invalid token: you token is either invalid or it has been expired"),
        }
    }
}