use crate::errors::ServerError;
use crate::modules::subscription_manager::SubscriptionManager;
use crate::redis_db::RedisDB;
use axum::extract::ws::WebSocket;
use std::sync::Arc;
use uuid::Uuid;

// структура модуля нотификации
pub struct NotificationModule {
    redis_db: Arc<RedisDB>,
    subscription_manager: Arc<SubscriptionManager>,
}

impl NotificationModule {
    pub fn new(redis_db: Arc<RedisDB>, subscription_manager: Arc<SubscriptionManager>) -> Self {
        NotificationModule {
            redis_db,
            subscription_manager,
        }
    }

    pub async fn notification_socket(
        &self,
        socket: WebSocket,
        user_uuid: Uuid,
    ) -> Result<(), ServerError> {
        // получение канала сообщений о новых подписках
        let subscription_change_channel_receiver = match self
            .subscription_manager
            .get_user_subscription_change_channel_receiver(user_uuid)
        {
            Some(receiver) => receiver,
            None => {
                return Err(ServerError::Broadcast(format!(
                    "не найден канал отправки пользователя {}",
                    user_uuid
                )))
            }
        };

        // получение подписок пользователя
        let user_subscriptions = self
            .subscription_manager
            .get_all_subscriptions_by_user_uuid(user_uuid)
            .await?;

        // начало передачи данных через WebSocket
        self.redis_db
            .handle_pub_sub(
                socket,
                user_subscriptions,
                subscription_change_channel_receiver,
                user_uuid,
            )
            .await?;

        Ok(())
    }
}
