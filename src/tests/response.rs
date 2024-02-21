use actix_web::{http::StatusCode, web, ResponseError};

use crate::response::{AppError, AppResponse, AppResult};

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
