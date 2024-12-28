// use crate::models::user::User;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn from_user_uuid(pool: &Pool<Postgres>, user_uuid: &Uuid, password_hash: &str) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("UPDATE password_history ph SET ph.user_id = u.id, password_hash=($1) FROM user u WHERE u.uuid=($2);")
        // .bind(user)
        .bind(password_hash)
        .bind(user_uuid)
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        println!("error: {:?}", err);
        return Err(err);
    } else {
        return Ok(());
    }
}
