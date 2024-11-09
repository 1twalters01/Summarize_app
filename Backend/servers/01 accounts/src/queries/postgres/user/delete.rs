use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn from_uuid(pool: &Pool<Postgres>, uuid: &Uuid) -> Result<(), sqlx::Error> {
    let user_delete_query = sqlx::query("Delete FROM users WHERE uuid=($1);")
        .bind(uuid)
        .execute(pool)
        .await;

    if let Err(err) = user_delete_query {
        return Err(err);
    } else {
        return Ok(());
    }
}
