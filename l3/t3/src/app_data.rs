use crate::data_model::{Message, MessagePayload, Room, RoomPayload, User, UserPayloadHashed};
use dashmap::DashMap;
use std::error::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{Sender};
use tracing::info;
use uuid::Uuid;

// структура данных сервера
pub struct AppData {
    // DashMap: логин - объект пользователя
    users: DashMap<String, User>,
    // DashMap: Uuid комнаты - объект комнаты
    rooms: DashMap<Uuid, Room>,
    // DashMap: Uuid комнаты - RwLock с вектором сообщений
    messages: DashMap<Uuid, Arc<RwLock<Vec<Message>>>>,
    // счётчик пользователей
    user_count: AtomicUsize,
}

impl AppData {
    pub fn new() -> Self {
        let users: DashMap<String, User> = DashMap::new();
        let rooms: DashMap<Uuid, Room> = DashMap::new();
        let messages: DashMap<Uuid, Arc<RwLock<Vec<Message>>>> = DashMap::new();
        let user_count = AtomicUsize::new(0);

        AppData {
            users,
            rooms,
            messages,
            user_count
        }
    }

    // возврат пользователя по логину из базы
    pub fn get_user_by_login(&self, login: &str) -> Option<User> {
        self.users.get(login).map(|user| user.clone())
    }

    // добавление нового пользователя в данные
    pub fn insert_new_user(
        &self,
        user_payload: UserPayloadHashed,
    ) {
        let user_login = user_payload.login.clone();
        let new_user = User {
            login: user_payload.login.clone(),
            password_hash: user_payload.password_hash,
            room_uuid: None,
        };

        // добавление нового пользователя и инкрементация счётчика пользователей
        self.users.insert(user_payload.login, new_user);
        self.user_count.fetch_add(1, Ordering::SeqCst);

        info!("пользователь с логином {} добавлен в данные", &user_login);
    }

    // добавление новой комнаты в данные и запуск трэда обработки сообщений
    pub fn create_new_room(&self, room_payload: RoomPayload) {
        // создание нового id комнаты
        let room_uuid = Uuid::new_v4();
        // создание канала отправки и получения сообщений
        let (tx, mut rx) = mpsc::channel(50);

        let new_room = Room {
            room_uuid,
            active: true,
            name: room_payload.name,
            tx,
        };

        // добавление новой комнаты в данные
        self.rooms.insert(room_uuid, new_room);

        // создание вектора сообщений, клонирование Arc для добавления в трэды
        let messages_vec: Arc<RwLock<Vec<Message>>> = Arc::new(RwLock::new(Vec::new()));
        let messages_vec_clone = Arc::clone(&messages_vec);

        // добавление вектора сообщений в данные
        self.messages.insert(room_uuid, messages_vec);

        info!(
            "комната с uuid: {} добавлена в данные и запущена обработка входящих сообщений",
            room_uuid
        );

        // tokio-task для обработки входящих сообщений
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                let mut messages = messages_vec_clone.write().expect("не удалось разблокировать RwLock");

                info!(
                    "Пользователь c логином: {} написал сообщение ' {} ' в канал с UUID: {} и сообщение было обработано трэдом",
                    &message.user_login, &message.message_text, room_uuid
                );

                messages.push(message);
            }

            info!("трэд обработки сообщений комнаты {} прекратил работу", room_uuid)
        });
    }

    // возврат комнаты по uuid из базы
    pub fn get_room_by_uuid(&self, room_uuid: Uuid) -> Option<Room> {
        self.rooms.get(&room_uuid).map(|room| room.clone())
    }

    // получение всех сообщений из данных о комнате
    pub fn get_room_messages(&self, room_uuid: Uuid) -> Vec<Message> {
        let message_guard = self.messages.get(&room_uuid).expect("ссылка на RwLock отсутсвует");

        let messages = message_guard.read().expect("не удалось прочитать из RwLock");
        messages.clone()
    }

    // возврат счётчика пользователей
    pub fn get_user_count(&self) -> &AtomicUsize {
        &self.user_count
    }

    // отправка сообщения
    pub fn send_message(&self, message_payload: MessagePayload, login: String, tx: Sender<Message>) -> Result<(), Box<dyn Error>> {
        let message_uuid = Uuid::new_v4();
        let new_message = Message {
            message_uuid,
            user_login: login,
            message_text: message_payload.message_text,
            created_at: chrono::Utc::now().timestamp()
        };

        // попытка отправить сообщение
        tx.try_send(new_message)?;

        info!("Сообщение с uuid: {} создано и отправлено в трэд обработки", message_uuid);

        Ok(())
    }

    pub fn change_channel(&self, mut user: User, room_uuid: Uuid) -> Option<()> {
        let login = user.login.clone();

        // проверка на адекватность запроса
        if user.room_uuid == Some(room_uuid) {
            return None
        };

        // смена комнаты в данных пользователя
        user.room_uuid = Some(room_uuid);
        self.users.insert(login.clone(), user);

        info!("пользователь под логином {} вошел в комнату {}", login, room_uuid);

        Some(())
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self::new()
    }
}
