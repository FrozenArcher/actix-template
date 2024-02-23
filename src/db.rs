use crate::config::AppConfig;
use derive_more::{Display, Error};
use log::info;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::io;

#[derive(Clone)]
pub enum AppDB {
    Postgres(Pool<Postgres>),
    Mock,
}

impl AppDB {
    pub async fn postgres(config: &AppConfig) -> io::Result<Self> {
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
            Ok(Self::Postgres(pool))
        } else {
            let msg = format!("failed to connect to database, config {:?}", config);
            Err(io::Error::other(msg))
        }
    }
    pub fn mock() -> Self {
        Self::Mock
    }
}

impl AppDB {
    pub async fn test_db(&self) -> DBResult<()> {
        match self {
            Self::Postgres(pool) => {
                let row: (i64,) = sqlx::query_as("SELECT $1")
                    .bind(150_i64)
                    .fetch_one(pool)
                    .await?;

                assert_eq!(row.0, 150);
                Ok(())
            }
            _ => Err(DBError::Unimplemented),
        }
    }
}

pub type DBResult<T> = std::result::Result<T, DBError>;

#[derive(Debug, Error, Display)]
pub enum DBError {
    SqlxError(sqlx::Error),
    Unimplemented,
}

impl From<sqlx::Error> for DBError {
    fn from(err: sqlx::Error) -> Self {
        Self::SqlxError(err)
    }
}
