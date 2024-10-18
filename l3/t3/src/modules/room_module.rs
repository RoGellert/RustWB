use crate::app_data::AppData;
use crate::data_model::{Message, MessagePayload, RoomPayload, Validate};
use crate::errors::ServerError;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

// модуль комнат
pub struct RoomModule {
    app_data: Arc<AppData>,
}

impl RoomModule {
    // инициализация модуля комнат
    pub fn new(app_data: Arc<AppData>) -> Self {
        RoomModule { app_data }
    }

    // присоединение к комнате
    pub fn join_room(&self, login: String, room_uuid: Uuid) -> Result<(), ServerError> {
        // проверка существует ли комната в данных
        if self.app_data.get_room_by_uuid(&room_uuid).is_none() {
            return Err(ServerError::BusinessLogic(format!(
                "комната {} отсутствует в базе данных",
                room_uuid
            )));
        };

        // проверка существует ли пользователь в данных и получение его из данных
        let user = match self.app_data.get_user_by_login(&login) {
            None => {
                return Err(ServerError::BusinessLogic(format!(
                    "пользователь под логином {} отсутствует в базе данных",
                    login
                )))
            }
            Some(user) => user,
        };

        // изменение данных о пользователе
        match self.app_data.join_channel(user, room_uuid) {
            Err(err_text) => Err(ServerError::BusinessLogic(err_text)),
            Ok(()) => {
                info!(
                    "пользователь под логином {} вошел в комнату {}",
                    login, room_uuid
                );
                Ok(())
            }
        }
    }

    // выход из комнаты
    pub fn leave_room(&self, login: String) -> Result<(), ServerError> {
        // проверка существует ли пользователь в данных и получение его из данных
        let user = match self.app_data.get_user_by_login(&login) {
            None => {
                return Err(ServerError::BusinessLogic(format!(
                    "пользователь под логином {} отсутствует в базе данных",
                    login
                )))
            }
            Some(user) => user,
        };

        // проверка в комнате ли пользователь
        if user.room_uuid.is_none() {
            return Err(ServerError::BusinessLogic(format!(
                "пользователь с логином {} не находится в комнате",
                login
            )));
        };

        // изменение данных о пользователе
        let room_uuid = self.app_data.leave_channel(user);
        info!(
            "пользователь под логином {} вышел из комнаты {}",
            login, room_uuid
        );
        Ok(())
    }

    // создание комнаты и обработчика сообщений в комнате
    pub fn create_room(&self, room_payload: RoomPayload) -> Result<Uuid, ServerError> {
        // валидация
        if let Err(text) = room_payload.is_valid() {
            return Err(ServerError::Validation(text));
        }

        // создание комнаты и трэда обработки сообщений
        let room_uuid = self.app_data.create_new_room(room_payload);

        info!(
            "комната с uuid {} добавлена в данные и запущена обработка входящих сообщений",
            room_uuid
        );

        Ok(room_uuid)
    }

    pub fn send_message(
        &self,
        message_payload: MessagePayload,
        login: String,
    ) -> Result<Uuid, ServerError> {
        // валидация
        if let Err(text) = message_payload.is_valid() {
            return Err(ServerError::Validation(text));
        }

        // получение пользователя из базы
        let user = match self.app_data.get_user_by_login(&login) {
            None => {
                return Err(ServerError::BusinessLogic(format!(
                    "пользователь под логином {} отсутствует в базе данных",
                    login
                )))
            }
            Some(user) => user,
        };

        // проверка в комнате ли пользователь
        if user.room_uuid.is_none() {
            return Err(ServerError::BusinessLogic(format!(
                "пользователь с логином {} не находится в комнате",
                login
            )));
        };

        // получение комнаты из данных
        let room_uuid = user.room_uuid.unwrap();
        let room = match self.app_data.get_room_by_uuid(&room_uuid) {
            None => {
                return Err(ServerError::BusinessLogic(format!(
                    "комната {} отсутствует в базе данных",
                    room_uuid
                )))
            }
            Some(room) => room,
        };

        // отправка сообщения в трэд обработки через канал отправки сообщений
        let message_uuid = match self
            .app_data
            .send_message(message_payload, login, room.tx.clone())
        {
            Err(text) => return Err(ServerError::SendingToThread(text)),
            Ok(message_uuid) => message_uuid,
        };

        Ok(message_uuid)
    }

    pub fn get_messages_by_room_uuid(
        &self,
        room_uuid: &Uuid,
        login: String,
    ) -> Result<Vec<Message>, ServerError> {
        // получение сообщений из данных
        let messages = self.app_data.get_room_messages(room_uuid)?;

        // проверка на то пуст ли вектор
        match messages.is_empty() {
            true => Err(ServerError::NotFound(format!(
                "комната {} отсутствует в базе данных",
                room_uuid
            ))),
            false => {
                info!(
                    "пользователь c логином {} получил все сообщения из комнаты c uuid {}",
                    login, room_uuid
                );
                Ok(messages)
            }
        }
    }

    pub fn get_messages_from_curr_room(&self, login: String) -> Result<Vec<Message>, ServerError> {
        // получение пользователя из базы
        let user = match self.app_data.get_user_by_login(&login) {
            None => {
                return Err(ServerError::BusinessLogic(format!(
                    "пользователь под логином {} отсутствует в базе данных",
                    login
                )))
            }
            Some(user) => user,
        };

        // проверка в комнате ли пользователь
        if user.room_uuid.is_none() {
            return Err(ServerError::BusinessLogic(format!(
                "пользователь с логином {} не находится в комнате",
                login
            )));
        };

        // получение cообщений из данных
        self.get_messages_by_room_uuid(&user.room_uuid.unwrap(), login)
    }

    pub fn get_user_count(&self, login: String) -> usize {
        info!(
            "пользователь c логином {} получил общее количество пользователей",
            login
        );
        self.app_data.get_user_count()
    }
}
