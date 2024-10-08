CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    user_uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    login VARCHAR,
    password VARCHAR,
    name VARCHAR
);

CREATE TABLE posts (
    post_uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_uuid REFERENCES users(user_uuid),
    text VARCHAR,
    like_count INT
);

CREATE TABLE user_likes (
    user_uuid UUID PRIMARY REFERENCES users(order_uid),
    post_uuid UUID PRIMARY REFERENCES posts(post_uuid)
);