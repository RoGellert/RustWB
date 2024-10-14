use crate::data_types::{User, UserPayloadHashed};
use crate::pg_db::PostgresDB;
use std::error::Error;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserModule {
    postgres_db: Arc<PostgresDB>,
}

impl UserModule {
    // инициализация модуля модели пользователей
    pub fn new(postgres_db: Arc<PostgresDB>) -> Self {
        UserModule { postgres_db }
    }

    // добавление пользователя
    pub async fn insert_user(
        &self,
        user_payload_hashed: UserPayloadHashed,
    ) -> Result<(), Box<dyn Error>> {
        self.postgres_db.insert_user(user_payload_hashed).await
    }

    // получение пользователя из базы
    pub async fn get_user_by_login(&self, login: &str) -> Result<Option<User>, Box<dyn Error>> {
        self.postgres_db.get_user_by_login(login).await
    }
}
