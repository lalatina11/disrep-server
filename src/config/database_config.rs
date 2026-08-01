use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

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
    pub async fn establish_connection() -> anyhow::Result<PgPool> {
        let db_url = DatabaseConfig::new().url;
        // Production-ready pool configuration
        let pool = PgPoolOptions::new()
            .max_connections(50)
            .acquire_timeout(Duration::from_secs(3))
            .idle_timeout(Duration::from_secs(10))
            .connect(&db_url)
            .await?;

        Ok(pool)
    }

    pub async fn run_migrations() -> anyhow::Result<()> {
        let pool = Database::establish_connection()
            .await
            .expect("Cannot connect into db");
        // Reads the `./migrations` folder at compile time and bundles it!
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(())
    }
}
