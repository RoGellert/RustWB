use tokio::sync::mpsc::Sender;
use uuid::Uuid;

pub trait Validate {
    fn is_valid(&self) -> Result<(), String>;
}

#[derive(Clone)]
pub struct User {
    pub login: String,
    pub password_hash: String,
    pub room_uuid: Option<Uuid>,
}

pub struct UserPayload {
    pub login: String,
    pub password: String,
}

// валидация
impl Validate for UserPayload {
    fn is_valid(&self) -> Result<(), String> {
        if self.login.len() > 50 || self.login.is_empty() {
            return Err("логин не может быть пустым или длиннее чем 50 символов".to_string())
        }

        if self.password.len() > 50 || self.password.is_empty() {
            return Err("пароль не может быть пустым или длиннее чем 50 символов".to_string())
        }

        Ok(())
    }
}

pub struct UserPayloadHashed {
    pub login: String,
    pub password_hash: String,
}

#[derive(Clone)]
pub struct Message {
    pub message_uuid: Uuid,
    pub user_login: String,
    pub message_text: String,
    pub created_at: i64,
}

pub struct MessagePayload {
    pub message_text: String,
}

// валидация
impl Validate for MessagePayload {
    fn is_valid(&self) -> Result<(), String> {
        if self.message_text.len() > 250 || self.message_text.is_empty() {
            return Err("текст сообщения не может быть пустым или содержать больше чем 250 символов".to_string())
        }

        Ok(())
    }
}


#[derive(Clone)]
pub struct Room {
    pub room_uuid: Uuid,
    pub name: String,
    pub active: bool,
    // приёмник передачи сообщений в канал
    pub tx: Sender<Message>,
}

pub struct RoomPayload {
    pub name: String,
}

// валидация
impl Validate for  RoomPayload {
    fn is_valid(&self) -> Result<(), String> {
        if self.name.len() > 25 || self.name.is_empty() {
            return Err("название коменаты не может быть пустым или содержать больше чем 25 символов".to_string())
        }

        Ok(())
    }
}
