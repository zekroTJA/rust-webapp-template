use crate::{
    error::{Error, Result},
    models::User,
};
use futures::TryStreamExt;
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

    pub async fn users(&self) -> Result<Vec<User>> {
        let mut rows =
            sqlx::query_as(r#"SELECT "id", "registered_at" FROM "user""#).fetch(&self.pool);

        let mut res = vec![];
        while let Some(user) = rows.try_next().await? {
            res.push(user);
        }

        Ok(res)
    }

    pub async fn user(&self, id: &str) -> Result<Option<User>> {
        let res = sqlx::query_as(r#"SELECT "id", "registered_at" FROM "user" WHERE "id" = $1"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(res)
    }

    pub async fn add_user(&self, user: &User) -> Result<()> {
        sqlx::query(r#"INSERT INTO "user" ("id", "registered_at") VALUES ($1, $2)"#)
            .bind(&user.id)
            .bind(user.registered_at)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
