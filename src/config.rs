use std::env;

#[derive(Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_database: String,
}

fn from_env(field: &'static str) -> String {
    let suf = " not found in `.env`";
    env::var(field).expect(&[field, suf].join(" "))
}

impl AppConfig {
    pub fn default() -> Self {
        // import settings from environment variables
        let host = from_env("HOST");
        let portstr = from_env("PORT");
        let db_user = from_env("DB_USER");
        let db_host = from_env("DB_HOST");
        let db_database = from_env("DB_DATABASE");
        let db_password = from_env("DB_PASSWORD");

        // parse portstr to u16
        let port = portstr
            .parse::<u16>()
            .expect(&["failed to parse value of `PORT`:", &portstr].join(" "));

        Self {
            host,
            port,
            db_host,
            db_user,
            db_database,
            db_password,
        }
    }
}
