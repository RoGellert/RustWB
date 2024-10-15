use serde::{Deserialize, Serialize};
use uuid::Uuid;

// структура пользователя
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_uuid: Uuid,
    pub login: String,
    pub password_hash: String,
}

// структура передаваемая в ручки регистрации и авторизации
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPayload {
    pub login: String,
    pub password: String,
}

// валидация
impl UserPayload {
    pub fn is_valid(&self) -> Result<(), String> {
        if self.login.len() > 50 || self.login.is_empty() {
            return Err("логин не может быть пустым или длиннее чем 50 символов".to_string())
        }

        if self.password.len() > 50 || self.password.is_empty() {
            return Err("пароль не может быть пустым или длиннее чем 50 символов".to_string())
        }

        Ok(())
    }
}

// структура с логином и хешированным паролем
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPayloadHashed {
    pub login: String,
    pub password_hash: String,
}

// структура поста
#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub post_uuid: Uuid,
    pub user_uuid: Uuid,
    pub post_text: String,
    pub like_count: u32,
}

// структура передаваемая в ручку создания поста
#[derive(Debug, Serialize, Deserialize)]
pub struct PostPayload {
    pub post_text: String,
}

// валидация
impl PostPayload {
    pub fn is_valid(&self) -> Result<(), String> {
        if self.post_text.len() > 250 || self.post_text.is_empty() {
            return Err("текст поста не может быть пустым или содержать больше чем 250 символов".to_string())
        }

        Ok(())
    }
}

// структура лайка пользователя
#[derive(Debug, Serialize, Deserialize)]
pub struct UserLike {
    pub post_uuid: Uuid,
    pub user_uuid: Uuid,
}
