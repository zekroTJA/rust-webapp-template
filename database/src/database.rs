use crate::error::{Error, Result};
use sqlx::PgPool;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(dsn: &str) -> Result<Self> {
        let pool = PgPool::connect(dsn)
            .await
            .map_err(Error::ConnectionFailed)?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }
}
