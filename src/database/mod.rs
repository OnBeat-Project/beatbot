pub mod models;
pub mod queries;

use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::time::Duration;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        if !std::path::Path::new("data").exists() {
            std::fs::create_dir("data")?;
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(database_url)
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
