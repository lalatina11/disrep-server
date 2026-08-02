use crate::{
    config::database_config::Database,
    models::user_model::{NewUser, UserModel},
    schema::users as users_table,
};

use diesel::{prelude::*, result::Error};
use uuid::Uuid;

pub struct UserService;
impl UserService {
    pub async fn create_user(payload: NewUser) -> Result<UserModel, String> {
        let conn = &mut Database::establish_connection();

        let query: Result<UserModel, Error> = diesel::insert_into(users_table::table)
            .values(payload)
            .returning(UserModel::as_returning())
            .get_result(conn);

        if let Ok(data) = query {
            return Ok(data);
        }

        Err("Failed to create a new user".to_string())
    }

    pub async fn get_user_by_id(user_id: Uuid) -> Result<UserModel, String> {
        use crate::schema::users::dsl::*;
        let conn = &mut Database::establish_connection();
        let query: Result<UserModel, Error> = users
            .find(user_id)
            .select(UserModel::as_select())
            .first::<UserModel>(conn);
        if let Ok(res) = query {
            return Ok(res);
        }

        Err("Failed to get User".to_string())
    }
}
