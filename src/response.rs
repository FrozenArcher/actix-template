use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    web, HttpResponse, Result,
};
use derive_more::{Display, Error};
use log::warn;
use serde::Serialize;

use crate::db::DBError;

/// Result type returned by handler functions.
pub type AppResult<T> = actix_web::Result<web::Json<OkModel<T>>, AppError>;

/// # Error JSON response model
#[derive(Serialize)]
pub struct ErrModel {
    pub success: bool,
    pub err: &'static str,
}

/// # Success JSON response model
#[derive(Serialize)]
pub struct OkModel<T>
where
    T: Serialize,
{
    pub success: bool,
    pub data: T,
}

/// # App Errors
#[derive(Debug, Display, Error, Serialize)]
pub enum AppError {
    Invalid { err: &'static str },
    InternalError,
}

impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Invalid { .. } => StatusCode::BAD_REQUEST,
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        let mut builder = HttpResponse::build(self.status_code());
        let resp = builder.insert_header(ContentType::json());
        match *self {
            Self::Invalid { err } => resp.json(ErrModel {
                success: false,
                err,
            }),
            Self::InternalError => resp.json(ErrModel {
                success: false,
                err: "502 internal server error",
            }),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        warn!("{}", err);
        Self::InternalError
    }
}

impl From<DBError> for AppError {
    fn from(err: DBError) -> Self {
        warn!("{}", err);
        Self::InternalError
    }
}

/// App response types
///
/// # Examples
///
/// ## Success
///
/// ```no_run
///#[get("/ping")]
///pub async fn ping() -> AppResult<PingResponse> {
///    AppResponse::Success(PingResponse { msg: "pong" }).response()
///}
///```
///
/// ## Internal server error
///
///```no_run
///#[get("/test/internal")]
///pub async fn test_internal() -> AppResult<()> {
///    AppResponse::InternalError.response()
///}
///```
///
/// ## Bad request
///
///```no_run
///#[get("/test/invalid")]
///pub async fn test_invalid() -> AppResult<()> {
///    AppResponse::Invalid("test invalid").response()
///}
/// ```
#[derive(Serialize, Debug, Display)]
pub enum AppResponse<T>
where
    T: Serialize,
{
    Success(T),
    Invalid(&'static str),
    InternalError(&'static str),
}

impl<T> AppResponse<T>
where
    T: Serialize,
{
    pub fn response(self) -> Result<web::Json<OkModel<T>>, AppError> {
        match self {
            Self::Success(data) => Ok(web::Json(OkModel {
                success: true,
                data,
            })),
            Self::Invalid(err) => Err(AppError::Invalid { err }),
            Self::InternalError(err) => {
                warn!("{}", err);
                Err(AppError::InternalError)
            }
        }
    }
}
