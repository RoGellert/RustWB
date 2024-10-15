### Для запуска через докер: 
```
docker compose up --build
```

### Структура базы данных

```SQL
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- данные пользователя
CREATE TABLE users (
    user_uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    login VARCHAR UNIQUE, -- логин пользователя
    password_hash VARCHAR -- пароль пользователя
);

-- посты пользователей
CREATE TABLE posts (
    post_uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_uuid UUID REFERENCES users(user_uuid),
    post_text VARCHAR, -- текст поста
    like_count INT -- счётчик лайков поста (намеренно денормализованный)
);

-- many-to-many лайки пользователей
CREATE TABLE user_likes (
    user_uuid UUID REFERENCES users(user_uuid),
    post_uuid UUID REFERENCES posts(post_uuid) ON DELETE CASCADE,
    PRIMARY KEY (
        user_uuid,
        post_uuid
    )
);
```

### Структура проекта

| файл                | назначение                                                                                             |
|---------------------|--------------------------------------------------------------------------------------------------------|
| main                | инициализация модулей и запуск сервера                                                                 |
| pg_db               | модуль базы данных с декларацией методов записи/получения объектов                                     |
| modules/auth_module | регистрация, получение jwt через log-in и миддлвара для авторизации запросов                           |
| modules/post_module | модуль для работы с постами                                                                            |
| modules/user_module | вспомогательный модуль для работы с данными пользователя, фактически обёртка вокруг модуля базы данных |
| config              | структуры и методы работы с конфигурацией и чтения переменных окружения                                |
| controller          | функции хэндлеров                                                                                      |
| errors              | ошибки http                                                                                            |
| data_types          | типы данных и их валидация                                                                             |



