CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- данные пользователя
CREATE TABLE users (
    user_uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    login VARCHAR, -- логин пользователя
    password_hash VARCHAR, -- пароль пользователя
);

-- посты пользователей
CREATE TABLE posts (
    post_uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_uuid REFERENCES users(user_uuid),
    post_text VARCHAR, -- текст поста
    like_count INT -- счётчик лайков поста (намеренно денормализованный)
);

-- many-to-many лайки пользователей
CREATE TABLE user_likes (
    user_uuid UUID PRIMARY REFERENCES users(order_uid),
    post_uuid UUID PRIMARY REFERENCES posts(post_uuid)
);