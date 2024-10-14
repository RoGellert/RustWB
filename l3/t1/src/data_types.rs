use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub user_uuid: Uuid,
    pub login: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPayload {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPayloadHashed {
    pub login: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub post_uuid: Uuid,
    pub post_text: String,
    pub like_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostPayload {
    pub post_text: String,
}
