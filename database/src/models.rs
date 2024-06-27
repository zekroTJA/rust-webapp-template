use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct User {
    pub id: String,
    pub registered_at: DateTime<Utc>,
}

impl User {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self {
            id: id.into(),
            registered_at: Utc::now(),
        }
    }
}
