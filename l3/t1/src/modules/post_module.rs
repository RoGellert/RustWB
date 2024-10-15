use crate::data_types::{Post, PostPayload};
use crate::errors::ServerError;
use crate::pg_db::PostgresDB;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

pub struct PostModule {
    postgres_db: Arc<PostgresDB>,
}

impl PostModule {
    // инициализация модуля модели постов
    pub fn new(postgres_db: Arc<PostgresDB>) -> Self {
        PostModule { postgres_db }
    }

    // добавление поста в базу данных
    pub async fn create_post(
        &self,
        user_uuid: Uuid,
        post_payload: PostPayload,
    ) -> Result<(), ServerError> {
        // валидация
        if let Err(text) = post_payload.is_valid() {
            return Err(ServerError::Validation(text));
        }

        match self.postgres_db.insert_post(user_uuid, post_payload).await {
            Ok(()) => {
                info!("пользователь с uuid {} добавил новое сообщение", user_uuid);
                Ok(())
            },
            Err(err) => Err(ServerError::Postgres(err)),
        }
    }

    // получение поста из базы данных
    pub async fn get_post_by_uuid(&self, post_uuid: Uuid) -> Result<Post, ServerError> {
        match self.postgres_db.get_post_by_uuid(post_uuid).await {
            Ok(Some(post)) => {
                info!("пост с uuid {} получен из базы данных", post_uuid);
                Ok(post)
            },
            Ok(None) => Err(ServerError::NotFound(format!(
                "не найден пост с uuid: {}",
                post_uuid
            ))),
            Err(err) => Err(ServerError::Postgres(err)),
        }
    }

    // проверка на наличие поста в базе данных, в случае отсутствия - возвращает ошибку
    async fn check_post_existence(&self, post_uuid: Uuid) -> Result<(), ServerError> {
        match self.postgres_db.get_post_by_uuid(post_uuid).await {
            Ok(None) => Err(ServerError::NotFound(format!(
                "не найден пост с uuid: {}",
                post_uuid
            ))),
            Err(err) => Err(ServerError::Postgres(err)),
            Ok(Some(_)) => Ok(())
        }
    }

    // удаление поста из базы данных, в случае отсутствия - возвращает ошибку
    pub async fn delete_post_by_uuid(&self, post_uuid: Uuid) -> Result<(), ServerError> {
        // проверка на наличие поста в базе данных
        self.check_post_existence(post_uuid).await?;

        // удаление поста из базы
        match self.postgres_db.delete_post_by_uuid(post_uuid).await {
            Ok(()) => {
                info!("пост с uuid {} удалён из базы данных", post_uuid);
                Ok(())
            },
            Err(err) => Err(ServerError::Postgres(err)),
        }
    }

    // внесения лайка в базу и инкрементация количества лайков в посте
    pub async fn insert_like(&self, user_uuid: Uuid, post_uuid: Uuid) -> Result<(), ServerError> {
        // проверка на наличие поста в базе данных
        self.check_post_existence(post_uuid).await?;

        // проверка ставил ли пользователь уже лайк этому посту
        match self.postgres_db.get_user_like(user_uuid, post_uuid).await {
            Ok(Some(_)) => return Err(ServerError::BusinessLogic(format!(
                "лайк посту c uuid {} от пользователя c uuid {} уже стоит",
                post_uuid, user_uuid
            ))),
            Err(err) => return Err(ServerError::Postgres(err)),
            Ok(None) => {}
        }

        // добавление лайка в базу
        if let Err(err) = self.postgres_db.insert_user_to_like(user_uuid, post_uuid).await {
            return Err(ServerError::Postgres(err))
        }

        // инкрементация лайков в базе
        if let Err(err) = self.postgres_db.increment_likes_by_uuid(post_uuid).await {
            return Err(ServerError::Postgres(err))
        }

        info!("пользователь с uuid {} лайкнул пост {}", user_uuid, post_uuid);

        Ok(())
    }

    // получение всех постов пользователя из базы
    pub async fn get_all_posts_by_user_uuid(&self, user_uuid: Uuid) -> Result<Vec<Post>, ServerError> {
        match self.postgres_db.get_all_posts_by_user_uuid(user_uuid).await {
            Ok(Some(posts)) => {
                info!("пользователь {} получил все свои посты из базы", user_uuid);
                Ok(posts)
            },
            Ok(None) => Err(ServerError::NotFound(format!(
                "не найдены посты пользователя с uuid: {}",
                user_uuid
            ))),
            Err(err) => Err(ServerError::Postgres(err))
        }
    }
}
