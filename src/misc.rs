use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

use crate::{
    response::{AppResponse, AppResult},
    AppState,
};

#[derive(Serialize)]
struct PingResponse {
    msg: &'static str,
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, this is actix template API")
}

#[get("/ping")]
pub async fn ping() -> AppResult<PingResponse> {
    AppResponse::Success(PingResponse { msg: "pong" }).response()
}

/// defining an app
pub struct TestApp;

impl TestApp {
    pub fn app() -> actix_web::Scope {
        web::scope("/test")
            .service(test_internal)
            .service(test_invalid)
            .service(test_db)
    }
}

#[get("/internal")]
pub async fn test_internal() -> AppResult<()> {
    AppResponse::InternalError("some severe error").response()
}

#[get("/invalid")]
pub async fn test_invalid() -> AppResult<()> {
    AppResponse::Invalid("test invalid").response()
}

#[get("/db")]
pub async fn test_db(data: web::Data<AppState>) -> AppResult<&'static str> {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&data.db.pool)
        .await?;

    assert_eq!(row.0, 150);
    AppResponse::Success("success").response()
}
