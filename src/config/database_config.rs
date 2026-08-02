use diesel::{Connection, PgConnection};

use crate::config::env_config::EnvConfig;

pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        let url = EnvConfig::get("DATABASE_URL");
        Self { url }
    }
}

pub struct Database;

impl Database {
    pub fn establish_connection() -> PgConnection {
        let database_url = EnvConfig::get("DATABASE_URL");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
