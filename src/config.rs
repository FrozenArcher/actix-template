use std::env;

#[derive(Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub use_mock: bool,
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_database: String,
}

/// Reads an environment variable and return its value
///
/// If the value is not found, the program panics with a message.
fn from_env(field: &'static str) -> String {
    let suf = "not found in `.env`";
    env::var(field).expect(&[field, suf].join(" "))
}

impl AppConfig {
    pub fn default() -> Self {
        // import settings from environment variables
        let host = from_env("HOST");
        let portstr = from_env("PORT");

        // parse portstr to u16
        let port = portstr
            .parse::<u16>()
            .expect(&["failed to parse value of `PORT`:", &portstr].join(" "));

        // if read `Mock=1` then use mock and do not read database settings
        if let Ok(mock) = env::var("MOCK") {
            if mock == "1".to_string() {
                return Self {
                    host,
                    port,
                    use_mock: true,
                    db_database: "".to_string(),
                    db_host: "".to_string(),
                    db_password: "".to_string(),
                    db_user: "".to_string(),
                };
            }
        }

        let db_user = from_env("DB_USER");
        let db_host = from_env("DB_HOST");
        let db_database = from_env("DB_DATABASE");
        let db_password = from_env("DB_PASSWORD");

        Self {
            host,
            port,
            use_mock: false,
            db_host,
            db_user,
            db_database,
            db_password,
        }
    }
}
