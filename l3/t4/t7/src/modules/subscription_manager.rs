use std::collections::HashSet;
use crate::redis_db::RedisDB;
use std::sync::Arc;
use dashmap::DashMap;
use tokio::sync::broadcast::Sender;
use uuid::Uuid;
use crate::errors::ServerError;

// структура мэнеджера подписок
pub struct SubscriptionManager {
    redis_db: Arc<RedisDB>,
    // hash_map для отправки сообщений об изменениях в подписках
    channel_map: DashMap<Uuid, Sender<()>>,
}

impl SubscriptionManager {
    pub fn new(redis_db: Arc<RedisDB>) -> SubscriptionManager {
        SubscriptionManager {
            redis_db,
            channel_map: DashMap::<Uuid, Sender<()>>::new(),
        }
    }

    pub async fn subscribe(&self, user_uuid: Uuid, event_type: String) -> Result<(), ServerError> {
        // проверка подписан ли пользователь на уведомления этого типа
        match self.redis_db.get_all_subscriptions_by_user_uuid(user_uuid).await {
            Err(err) => return Err(ServerError::Redis(err)),
            Ok(Some(subscriptions)) => {
                let subscription_set: HashSet<String> = subscriptions.into_iter().collect();
                if subscription_set.contains(&event_type) {
                    return Err(ServerError::BusinessLogic(format!("Пользователь {} уже подписан на {}", user_uuid, event_type)))
                }
            },
            _ => {}
        };

        if let Err(err) = self.redis_db.subscribe(user_uuid, event_type).await {
            return Err(ServerError::Redis(err))
        }

        Ok(())
    }

    pub async fn get_all_subscriptions_by_user_uuid(&self, user_uuid: Uuid) -> Result<Vec<String>, ServerError> {
        let subscriptions = match self.redis_db.get_all_subscriptions_by_user_uuid(user_uuid).await {
            Ok(None) => return Err(ServerError::NotFound(format!("не найдены подписки пользователя: {}", user_uuid))),
            Ok(Some(subscriptions)) => subscriptions,
            Err(err) => return Err(ServerError::Redis(err))
        };

        Ok(subscriptions)
    }
}
