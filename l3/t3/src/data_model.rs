use tokio::sync::mpsc::Sender;
use uuid::Uuid;

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
