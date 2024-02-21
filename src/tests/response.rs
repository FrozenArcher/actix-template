use actix_web::{http::StatusCode, web, ResponseError};
use serde::Serialize;

use crate::response::{AppError, AppResponse, AppResult, OkModel};

#[test]
fn success() {
    let response: AppResult<&str> = AppResponse::Success("success").response();
    let web::Json(model) = response.unwrap();
    assert_eq!(model.success, true);
    assert_eq!(model.data, "success");
}

#[test]
fn bad_request() {
    let response: AppResult<()> = AppResponse::Invalid("bad").response();
    if let Err(e) = response {
        if let AppError::Invalid { err } = e {
            assert_eq!(err, "bad");
        }
        let resp = e.error_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    } else {
        panic!("response is not Err")
    }
}

#[test]
fn internal_server_error() {
    let response: AppResult<()> = AppResponse::InternalError("500").response();
    if let Err(e) = response {
        if let AppError::InternalError = e {}
        let resp = e.error_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}

#[test]
fn custom_response() {
    #[derive(Serialize)]
    struct RespBody {
        a: i32,
        b: f64,
        msg: &'static str,
    }
    let web::Json(model) = AppResponse::Success(RespBody {
        a: 1,
        b: 3.1415,
        msg: "Hello!",
    })
    .response()
    .unwrap();
    let OkModel { success, data } = model;
    assert!(success);
    let RespBody { a, b, msg } = data;
    assert_eq!(a, 1);
    assert_ne!(b, 9876.5432);
    assert_eq!(msg, "Hello!");
}
