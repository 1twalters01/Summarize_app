use crate::{models::user::User, queries::postgres::user::get::from_email};
use sqlx::{Pool, Postgres};

pub struct UserService {
    pool: Pool<Postgres>,
}

impl UserService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_user_from_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        from_email(&self.pool, &email).await
    }
}
