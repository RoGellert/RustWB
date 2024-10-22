use crate::errors::ServerError;
use crate::pg_db::PostgresDB;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

// структура пользователя
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub email: String,
}

// структура пользователя для изменения данных
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPayload {
    pub name: String,
    pub email: String,
}

pub struct UserModule {
    postgres_db: Arc<PostgresDB>,
}

impl UserModule {
    // инициализация модуля модели пользователей
    pub fn new(postgres_db: Arc<PostgresDB>) -> Self {
        UserModule { postgres_db }
    }

    // добавление пользователя
    pub async fn add_user(&self, user: User) -> Result<(), ServerError> {
        let user_id = user.user_id;
        match self.postgres_db.insert_user(user).await {
            Ok(()) => {
                info!("Добавлен новый пользователь с user_id: {}", user_id);
                Ok(())
            }
            Err(err) => Err(ServerError::Postgres(err)),
        }
    }

    // изменение данных пользователя
    pub async fn update_user(&self, user_id: i32, user_payload: UserPayload) -> Result<(), ServerError> {
        match self.postgres_db.update_user(user_id, user_payload).await {
            Ok(()) => {
                info!("Обновлены данные пользователя с user_id: {}", user_id);
                Ok(())
            }
            Err(err) => Err(ServerError::Postgres(err)),
        }
    }

    // удаление пользователя
    pub async fn delete_user(&self, user_id: i32) -> Result<(), ServerError> {
        match self.postgres_db.delete_user(user_id).await {
            Ok(()) => {
                info!("Удалён пользователь с user_id: {}", user_id);
                Ok(())
            }
            Err(err) => Err(ServerError::Postgres(err)),
        }
    }
}
