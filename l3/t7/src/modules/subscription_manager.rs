use crate::errors::ServerError;
use crate::redis_db::RedisDB;
use dashmap::DashMap;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};
use tracing::info;
use uuid::Uuid;

// структура мэнеджера подписок
pub struct SubscriptionManager {
    redis_db: Arc<RedisDB>,
    // hash_map для отправки сообщений об изменениях в подписках
    subscription_change_channel_map: DashMap<Uuid, Arc<Sender<String>>>,
}

impl SubscriptionManager {
    pub fn new(redis_db: Arc<RedisDB>) -> SubscriptionManager {
        SubscriptionManager {
            redis_db,
            subscription_change_channel_map: DashMap::<Uuid, Arc<Sender<String>>>::new(),
        }
    }

    pub async fn subscribe(&self, user_uuid: Uuid, event_type: String) -> Result<(), ServerError> {
        // проверка подписан ли пользователь на уведомления этого типа
        match self
            .redis_db
            .get_all_subscriptions_by_user_uuid(user_uuid)
            .await
        {
            Err(err) => return Err(ServerError::Redis(err)),
            Ok(Some(subscriptions)) => {
                // проверка подписан ли пользователь на данный тип событий
                let subscription_set: HashSet<String> = subscriptions.into_iter().collect();
                if subscription_set.contains(&event_type) {
                    return Err(ServerError::BusinessLogic(format!(
                        "Пользователь {} уже подписан на {}",
                        user_uuid, event_type
                    )));
                }

                // отправить сообщение пользователю о добавлении подписки в случае если он подлючен через WebSocket
                let subscription_change_channel =
                    match self.get_user_subscription_change_channel_sender(user_uuid) {
                        Some(channel) => channel,
                        None => {
                            return Err(ServerError::Broadcast(format!(
                                "не найден канал отправки пользователя {}",
                                user_uuid
                            )))
                        }
                    };
                subscription_change_channel
                    .send(event_type.clone())
                    .map_err(|_| {
                        ServerError::Broadcast(format!(
                            "не удалось отправить сообщение через канал отправки пользователя {}",
                            user_uuid
                        ))
                    })?;
            }
            Ok(None) => {
                // создание канала отправки подписок на события
                let (tx, _) = broadcast::channel(10);
                self.subscription_change_channel_map
                    .insert(user_uuid, Arc::new(tx));
            }
        };

        // добавление подписки в базу данных
        if let Err(err) = self.redis_db.subscribe(user_uuid, event_type.clone()).await {
            return Err(ServerError::Redis(err));
        }

        info!(
            "пользователь {} подписался на канал {}",
            user_uuid, event_type
        );

        Ok(())
    }

    // получение канала отправки broadcast channel пользователя для отправки сообщений о новых подписках в хендлер web-socket
    pub fn get_user_subscription_change_channel_sender(
        &self,
        user_uuid: Uuid,
    ) -> Option<Arc<Sender<String>>> {
        self.subscription_change_channel_map
            .get(&user_uuid)
            .map(|sender| Arc::clone(&sender))
    }

    // получение канала приёма broadcast channel пользователя для отправки сообщений о новых подписках в хендлер web-socket
    pub fn get_user_subscription_change_channel_receiver(
        &self,
        user_uuid: Uuid,
    ) -> Option<Receiver<String>> {
        self.subscription_change_channel_map
            .get(&user_uuid)
            .map(|sender| Arc::clone(&sender).subscribe())
    }

    // получение всех подписок пользователя
    pub async fn get_all_subscriptions_by_user_uuid(
        &self,
        user_uuid: Uuid,
    ) -> Result<Vec<String>, ServerError> {
        let subscriptions = match self
            .redis_db
            .get_all_subscriptions_by_user_uuid(user_uuid)
            .await
        {
            Ok(None) => {
                return Err(ServerError::NotFound(format!(
                    "не найдены подписки пользователя: {}",
                    user_uuid
                )))
            }
            Ok(Some(subscriptions)) => subscriptions,
            Err(err) => return Err(ServerError::Redis(err)),
        };

        info!(
            "пользователь {} получил свои подписки из базы данных",
            user_uuid
        );

        Ok(subscriptions)
    }
}
