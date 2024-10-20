use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

pub trait Validate {
    fn is_valid(&self) -> Result<(), String>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    // логин пользователя
    pub login: String,
    // хэш пароля
    pub password_hash: String,
    // id комнаты в которой находится пользователь либо None
    pub room_uuid: Option<Uuid>,
}

#[derive(Serialize, Deserialize)]
pub struct UserPayload {
    pub login: String,
    pub password: String,
}

// валидация
impl Validate for UserPayload {
    fn is_valid(&self) -> Result<(), String> {
        if self.login.len() > 50 || self.login.is_empty() {
            return Err("логин не может быть пустым или длиннее чем 50 символов".to_string());
        }

        if self.password.len() > 50 || self.password.is_empty() {
            return Err("пароль не может быть пустым или длиннее чем 50 символов".to_string());
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserPayloadHashed {
    pub login: String,
    pub password_hash: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Message {
    // id сообщения
    pub message_uuid: Uuid,
    // логин пользователя отправившего сообщение
    pub user_login: String,
    // текст сообщения
    pub message_text: String,
    // время отправки сообщения
    pub created_at: i64,
}

#[derive(Serialize, Deserialize)]
pub struct MessagePayload {
    pub message_text: String,
}

// валидация
impl Validate for MessagePayload {
    fn is_valid(&self) -> Result<(), String> {
        if self.message_text.len() > 250 || self.message_text.is_empty() {
            return Err(
                "текст сообщения не может быть пустым или содержать больше чем 250 символов"
                    .to_string(),
            );
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Room {
    // id комнаты
    pub room_uuid: Uuid,
    // имя пользователя
    pub name: String,
    // сообщения в канале
    pub messages: Arc<RwLock<Vec<Message>>>,
    // приёмник передачи сообщений в канал
    pub tx: Sender<Message>,
}

#[derive(Serialize, Deserialize)]
pub struct RoomPayload {
    pub name: String,
}

// валидация
impl Validate for RoomPayload {
    fn is_valid(&self) -> Result<(), String> {
        if self.name.len() > 25 || self.name.is_empty() {
            return Err(
                "название комнаты не может быть пустым или содержать больше чем 25 символов"
                    .to_string(),
            );
        }

        Ok(())
    }
}
