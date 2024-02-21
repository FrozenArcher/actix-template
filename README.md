# Actix Web Template

This project is a basic template for [`actix-web`](https://github.com/actix/actix-web),
using [`sqlx`](https://github.com/launchbadge/sqlx) and [`postgresql`](https://www.postgresql.org/)

## Features

The template has already done these for you:

- Global configuration using `dotenvy`;
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
    success: true,
    data: {
        // some data
    }
}
```

### failure

```json
{
    success: false,
    err: "some error"
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

Using the tmplate, you can easily write your services:

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

Or get access to your database:

```rust
use serde::Serialize;
use crate::{
    response::{AppResponse, AppResult},
    AppState,
};

#[get("/db")]
pub async fn test_db(data: web::Data<AppState>) -> AppResult<&'static str> {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&data.db.pool)
        .await?;

    assert_eq!(row.0, 150);
    AppResponse::Success("success").response()
}
```
