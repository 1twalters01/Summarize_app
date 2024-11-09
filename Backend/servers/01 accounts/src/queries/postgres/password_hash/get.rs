use crate::models::user::User;
use sqlx::{Pool, Postgres, Row};

pub async fn all_previous_from_user(
    pool: &Pool<Postgres>,
    user: &User,
) -> Result<Vec<String>, sqlx::Error> {
    let password_array_select_query = sqlx::query("").bind(user.get_uuid()).fetch_all(pool).await;

    match password_array_select_query {
        Err(err) => return Err(err),
        Ok(res) => {
            if res.len() == 0 {
                return Ok(Vec::new());
            }

            let password_hash_vec: Vec<String> = res[0].get("previous_password_hashes");
            return Ok(password_hash_vec);
        }
    }
}
