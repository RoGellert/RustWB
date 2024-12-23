use crate::errors::ServerError;
use crate::modules::subscription_manager::SubscriptionManager;
use crate::redis_db::RedisDB;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

// структура мэнеджера событий
pub struct EventManager {
    redis_db: Arc<RedisDB>,
    subscription_manager: Arc<SubscriptionManager>,
}

// стуктрура события для передачи в ручку
#[derive(Debug, Serialize, Deserialize)]
pub struct EventPayload {
    pub event_type: String,
    pub event_name: String,
}

// стуктрура события
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub event_uuid: Uuid,
    pub event_type: String,
    pub event_name: String,
}

impl Event {
    pub fn from_payload(event_payload: EventPayload) -> Self {
        Event {
            event_uuid: Uuid::new_v4(),
            event_name: event_payload.event_name,
            event_type: event_payload.event_type,
        }
    }
}

impl EventManager {
    pub fn new(redis_db: Arc<RedisDB>, subscription_manager: Arc<SubscriptionManager>) -> Self {
        EventManager {
            redis_db,
            subscription_manager,
        }
    }

    // добавление нового ивента в базу и нотификация
    pub async fn add_event(&self, event_payload: EventPayload) -> Result<(), ServerError> {
        let event = Event::from_payload(event_payload);
        let event_type = event.event_type.clone();
        let event_uuid = event.event_uuid;

        // сериализация
        let event_serialised = serde_json::to_string_pretty(&event).map_err(|_| {
            ServerError::Serialisation(format!("не удалось сериализовать : {:?}", event))
        })?;

        // добавление в базу
        if let Err(err) = self
            .redis_db
            .add_event(event, event_serialised.clone())
            .await
        {
            return Err(ServerError::Redis(err));
        }

        // уведомление пользователя через Redis PubSub
        if let Err(err) = self
            .redis_db
            .publish_message(event_type, event_serialised)
            .await
        {
            return Err(ServerError::Redis(err));
        }

        info!(
            "в базу данных добавлено новое событие с uuid: {}",
            event_uuid
        );

        Ok(())
    }

    // получение всех ивентов на которые подписан пользователь из базы
    pub async fn get_all_events_by_user_uuid(
        &self,
        user_uuid: Uuid,
    ) -> Result<Vec<String>, ServerError> {
        // получение подписок пользователя
        let subscriptions = self
            .subscription_manager
            .get_all_subscriptions_by_user_uuid(user_uuid)
            .await?;

        // получение событий из базы
        let mut event_strings: Vec<String> = Vec::new();
        for subscription in subscriptions {
            if let Ok(Some(events)) = self.redis_db.get_events_by_category(subscription).await {
                event_strings.extend(events);
            }
        }

        // проверка на наличие событий в базе
        if event_strings.is_empty() {
            return Err(ServerError::NotFound(format!(
                "не найдены события пользователя: {}",
                user_uuid
            )));
        }

        info!(
            "пользователь {} получил события на которые он подписан из базы данных",
            user_uuid
        );

        Ok(event_strings)
    }
}
