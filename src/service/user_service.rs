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

        let rec = sqlx::query!(
            r#"
        INSERT INTO users (id, email, display_name)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
            id,
            email,
            display_name,
        )
        .fetch_one(&pool)
        .await;

        if let Ok(res) = rec {
            let res = UserService::get_user_by_id(res.id).await;
            if let Ok(res) = res {
                return Ok(res);
            }
        }

        Err("Failed to insert user".to_string())
    }

    pub async fn get_user_by_id(id: String) -> anyhow::Result<UserModel, String> {
        let pool = Database::establish_connection()
            .await
            .map_err(|_| "Database connection failed".to_string())?;
        let res = sqlx::query_as!(
            UserModel,
            r#"SELECT * FROM users WHERE id = $1 LIMIT 1"#,
            id
        )
        .fetch_one(&pool)
        .await;

        if let Ok(res) = res {
            return Ok(res);
        }

        Err("Failed to get User".to_string())
    }
}
