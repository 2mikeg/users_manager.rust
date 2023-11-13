use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub created_at: DateTime<Utc>,
}
