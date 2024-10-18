use crate::data_model::{Message, MessagePayload, Room, RoomPayload, User, UserPayloadHashed};
use crate::errors::ServerError;
use dashmap::DashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tracing::info;
use uuid::Uuid;

// структура данных сервера
pub struct AppData {
    // DashMap: логин - объект пользователя
    users: DashMap<String, User>,
    // DashMap: Uuid комнаты - объект комнаты
    rooms: DashMap<Uuid, Room>,
    // счётчик пользователей
    user_count: AtomicUsize,
}

impl AppData {
    pub fn new() -> Self {
        let users: DashMap<String, User> = DashMap::new();
        let rooms: DashMap<Uuid, Room> = DashMap::new();
        let user_count = AtomicUsize::new(0);

        AppData {
            users,
            rooms,
            user_count,
        }
    }

    // возврат пользователя по логину из базы
    pub fn get_user_by_login(&self, login: &str) -> Option<User> {
        self.users.get(login).map(|user| user.clone())
    }

    // добавление нового пользователя в данные
    pub fn insert_new_user(&self, user_payload: UserPayloadHashed) {
        // создание нового пользователя
        let new_user = User {
            login: user_payload.login.clone(),
            password_hash: user_payload.password_hash,
            room_uuid: None,
        };

        // добавление нового пользователя и инкрементация счётчика пользователей
        self.users.insert(user_payload.login, new_user);
        self.user_count.fetch_add(1, Ordering::SeqCst);
    }

    // добавление новой комнаты в данные и запуск трэда обработки сообщений
    pub fn create_new_room(&self, room_payload: RoomPayload) -> Uuid {
        // создание нового id комнаты
        let room_uuid = Uuid::new_v4();
        // создание канала отправки и получения сообщений
        let (tx, mut rx) = mpsc::channel(50);

        // создание вектора сообщений
        let messages: Arc<RwLock<Vec<Message>>> = Arc::new(RwLock::new(Vec::new()));
        // клон для трэда
        let messages_clone = Arc::clone(&messages);

        // создание новой комнаты
        let new_room = Room {
            messages,
            room_uuid,
            name: room_payload.name,
            tx,
        };

        // добавление новой комнаты в данные
        self.rooms.insert(room_uuid, new_room);

        // tokio-task для обработки входящих сообщений
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                // попытка разблокировки RwLock
                let mut messages = messages_clone
                    .write()
                    .expect("не удалось разблокировать RwLock");

                info!(
                    "пользователь c логином: {} написал сообщение ' {} ' в канал с UUID: {} и сообщение было обработано трэдом",
                    &message.user_login, &message.message_text, room_uuid
                );

                // добавление сообщения в массив сообщений
                messages.push(message);
            }

            info!(
                "трэд обработки сообщений комнаты {} прекратил работу",
                room_uuid
            )
        });

        room_uuid
    }

    // возврат комнаты по uuid из базы
    pub fn get_room_by_uuid(&self, room_uuid: &Uuid) -> Option<Room> {
        self.rooms.get(room_uuid).map(|room| room.clone())
    }

    // получение всех сообщений из данных о комнате
    pub fn get_room_messages(&self, room_uuid: &Uuid) -> Result<Vec<Message>, ServerError> {
        // получение комнаты из базы данных
        let room = match self.get_room_by_uuid(room_uuid) {
            None => {
                return Err(ServerError::BusinessLogic(format!(
                    "комната c uuid {} отсутствует в базе данных",
                    room_uuid
                )))
            }
            Some(room) => room,
        };

        // получение сообщений из RwLock
        let x = match room.messages.read() {
            Ok(messages) => Ok(messages.clone()),
            Err(_) => Err(ServerError::Lock(format!(
                "не удалось прочитать содержимое RwLock комнаты {}",
                room_uuid
            ))),
        };
        x
    }

    // возврат счётчика пользователей
    pub fn get_user_count(&self) -> usize {
        self.user_count.load(Ordering::Relaxed)
    }

    // отправка сообщения
    pub fn send_message(
        &self,
        message_payload: MessagePayload,
        login: String,
        tx: Sender<Message>,
    ) -> Result<Uuid, String> {
        // новый Uuid сообщения
        let message_uuid = Uuid::new_v4();
        // новое сообщение
        let new_message = Message {
            message_uuid,
            user_login: login,
            message_text: message_payload.message_text,
            created_at: chrono::Utc::now().timestamp(),
        };

        // попытка отправить сообщение
        if tx.try_send(new_message).is_err() {
            return Err(format!(
                "нельзя войти в канал {} через приёмник отправки",
                message_uuid
            ));
        }

        Ok(message_uuid)
    }

    // смена канала
    pub fn join_channel(&self, mut user: User, room_uuid: Uuid) -> Result<(), String> {
        // проверка на адекватность запроса
        if user.room_uuid == Some(room_uuid) {
            return Err(format!(
                "нельзя войти в канал {} в котором уже присутствует пользователь под логином {}",
                room_uuid, user.login
            ));
        };

        // смена комнаты в данных пользователя
        user.room_uuid = Some(room_uuid);
        self.users.insert(user.login.clone(), user);

        Ok(())
    }

    // покидание канала
    pub fn leave_channel(&self, mut user: User) -> Uuid {
        // канал из которого вышел пользователь
        let room_uuid = user.room_uuid.unwrap();

        // смена комнаты в данных пользователя
        user.room_uuid = None;
        self.users.insert(user.login.clone(), user);

        room_uuid
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self::new()
    }
}
