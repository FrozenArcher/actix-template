# Actix Web Template

![gh_actions_rust](https://github.com/FrozenArcher/actix-template/actions/workflows/rust.yml/badge.svg)

This project is a basic template for [`actix-web`](https://github.com/actix/actix-web),
using [`sqlx`](https://github.com/launchbadge/sqlx) as database driver.

## Features

The template has already done these for you:

- Global configuration using `dotenvy`;
- Supporting multiple data sources at the same time, including mock data;
- Initializing a basic server;
- Connecting to a database;
- Defining response structure;
- Showing some example services/apps;
- Supporting `Logger`;

## Response structure

The template uses JSON as response:

### Success

```json
{
    "success": true,
    "data": {
        "foo": "bar"
    }
}
```

### Failure

```json
{
    "success": false,
    "err": "some error"
}
```

## Environment

The template makes use of `dotenvy` to manage environment variables:

```bash
# Unix systems
mv .env.example .env
```

And edit `.env` file:

```bash
# Log level
RUST_LOG=debug

# To enable `Mock` data source:
# MOCK=1
MOCK=0

# Service
HOST="127.0.0.1"
PORT=8080

# Database
DB_HOST="localhost"
DB_USER="postgres"
DB_DATABASE="some_db"
DB_PASSWORD="PASSWORD"
```

## Example code

### Responses

```rust
use serde::Serialize;
use crate::response::{AppResponse, AppResult};

#[derive(Serialize)]
struct PingResponse {
    msg: &'static str,
}

#[get("/ping")]
pub async fn ping() -> AppResult<PingResponse> {
    AppResponse::Success(PingResponse { msg: "pong" }).response()
}
```

### Get access to your database

- 1st. Choose your data source:

```rust
// main.rs
impl AppState {
    pub async fn new(config: &AppConfig) -> std::io::Result<AppState> {
        let db = AppDB::postgres(&config).await?;
        Ok(AppState { db })
    }
}
```

- 2nd. Acquire your data:

```rust
// define the method yourself.
impl AppDB {
    // give the result a type if you wish to use the data in your response
    pub async fn test_db(&self) -> DBResult<()> {
        match self {
            Self::Postgres(pool) => {   // pool is already `&Pool<Postgres>`
                let row: (i64,) = sqlx::query_as("SELECT $1")
                    .bind(150_i64)
                    .fetch_one(pool)
                    .await?;

                assert_eq!(row.0, 150);
                Ok(())
            }
            // if you don't want to implement for other data sources:
            _ => Err(DBError::Unimplemented),
        }
    }
}
```

- 3rd. Use the data in your response:

```rust
use serde::Serialize;
use crate::{
    response::{AppResponse, AppResult},
    AppState,
};

#[get("/db")]
pub async fn test_db(data: web::Data<AppState>) -> AppResult<&'static str> {
    // or `let some_data = data.db.some_method(args).await?;`
    data.db.test_db().await?;
    AppResponse::Success("Test for db is success").response()
}
```
