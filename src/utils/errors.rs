use actix_web::{error::ResponseError, HttpResponse, http::StatusCode};
use derive_more::Display;
use actix_threadpool;
use serde::Serialize;
use std::fmt;

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

#[derive(Debug)]
pub enum AppErrorType {
    RusotoError,
    DbError,
    PoolError,
    KeyAlreadyExists,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                ..
            } => message.clone(),
            AppError {
                message: None,
                error_type: AppErrorType::KeyAlreadyExists,
                ..
            } => "The requested item is already present".to_string(),
            AppError {
                message: None,
                error_type: AppErrorType::RusotoError,
                ..
            } => "There was an error communicating with S3".to_string(),
            AppError {
                message: None,
                error_type: AppErrorType::PoolError,
                ..
            } => "Cannot get the connection pool to the database".to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::KeyAlreadyExists => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError
        }
    }
}

impl<E> From<actix_threadpool::BlockingError<E>> for AppError
where
    E: std::fmt::Debug,
    E: Into<AppError>,
{
    fn from(error: actix_threadpool::BlockingError<E>) -> AppError {
        match error {
            actix_threadpool::BlockingError::Error(e) => e.into(),
            actix_threadpool::BlockingError::Canceled => AppError {message: None, cause: None, error_type: AppErrorType::DbError},
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(error: diesel::result::Error) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError
        }
    }
}

impl<E> From<rusoto_core::RusotoError<E>> for AppError
where
    E: std::fmt::Debug,
    E: Into<AppError>,
{
    fn from(_: rusoto_core::RusotoError<E>) -> AppError {
        AppError {message: None, cause: None, error_type: AppErrorType::RusotoError}
    }
}

impl From<rusoto_s3::GetObjectError> for AppError {
    fn from(error: rusoto_s3::GetObjectError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError
        }
    }
}

impl From<r2d2::Error> for AppError {
    fn from(error: r2d2::Error) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::PoolError
        }
    }
}