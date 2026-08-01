use sqlx::Row;

use crate::{config::database_config::Database, models::user_model::UserModel};

pub struct UserService;
impl UserService {
    pub async fn create_user(
        id: String,
        email: String,
        display_name: String,
    ) -> anyhow::Result<UserModel, String> {
        let pool = Database::establish_connection()
            .await
            .map_err(|_| "Database connection failed".to_string())?;

        let rec = sqlx::query(
            r#"
    INSERT INTO users (id, email, display_name)
    VALUES ($1, $2, $3)
    RETURNING id
    "#,
        )
        .bind(id)
        .bind(email)
        .bind(display_name)
        .fetch_one(&pool)
        .await;

        if let Ok(row) = rec {
            let id: String = row.get("id");
            let find_user = UserService::get_user_by_id(id).await;
            if let Ok(user_model) = find_user {
                return Ok(user_model);
            } else {
                return Err("User is not inserted properly".to_string());
            }
        }

        Err("Failed to insert user".to_string())
    }

    pub async fn get_user_by_id(id: String) -> anyhow::Result<UserModel, String> {
        let pool = Database::establish_connection()
            .await
            .map_err(|_| "Database connection failed".to_string())?;
        let res = sqlx::query_as::<_, UserModel>(r#"SELECT * FROM users WHERE id = $1 LIMIT 1"#)
            .bind(id)
            .fetch_one(&pool)
            .await;

        if let Ok(res) = res {
            return Ok(res);
        }

        Err("Failed to get User".to_string())
    }
}
