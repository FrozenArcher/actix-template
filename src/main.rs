mod config;
mod db;
mod misc;
mod response;

#[cfg(test)]
mod tests;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use config::AppConfig;
use db::AppDB;
use dotenvy::dotenv;
use log::info;
use misc::*;

#[derive(Clone)]
pub struct AppState {
    pub db: AppDB,
}

impl AppState {
    pub async fn new(config: &AppConfig) -> std::io::Result<AppState> {
        let db: AppDB;
        if config.use_mock {
            info!("using `Mock` data source");
            db = AppDB::mock();
        } else {
            db = AppDB::postgres(config).await?;
        }
        Ok(AppState { db })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("please create `.env` file");
    env_logger::init();

    let config = AppConfig::default();
    info!("Server running at http://{}:{}", config.host, config.port);

    let state = AppState::new(&config).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .service(index)
            .service(ping)
            .service(TestApp::app())
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
