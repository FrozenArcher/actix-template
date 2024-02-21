use crate::config::AppConfig;
use log::info;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::io;

#[derive(Clone)]
pub struct AppDB {
    pub pool: Pool<Postgres>,
}

impl AppDB {
    pub async fn new(config: &AppConfig) -> io::Result<Self> {
        let db_url = format!(
            "postgres://{}:{}@{}/{}",
            config.db_user, config.db_password, config.db_host, config.db_database
        );

        let pool_result = PgPoolOptions::new()
            .max_connections(4)
            .connect(&db_url)
            .await;

        if let Ok(pool) = pool_result {
            info!(
                "connected to database {} at {}",
                config.db_database, config.db_host
            );
            Ok(AppDB { pool })
        } else {
            let msg = format!("failed to connect to database, config {:?}", config);
            Err(io::Error::other(msg))
        }
    }
}
