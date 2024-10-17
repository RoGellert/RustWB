//! конфиг баз данных
use dotenv::dotenv;
use std::env;

// структура конфига авторизации
#[derive(Clone)]
pub struct AuthConfig {
    pub jwt_expiry_time: i64,
    pub server_encoding_key: String,
}

impl AuthConfig {
    // инициализация конфига авторизации и загрузка переменных окружения
    pub fn new() -> Self {
        // загрузка переменных окружения
        dotenv().ok();

        // срок истечения токена JWT в секундах
        let jwt_expiry_time = env::var("JWT_EXPIRY_TIME")
            .expect("JWT_EXPIRY_TIME не найден в переменных окружения")
            .parse::<i64>()
            .expect("неверное количество секунд истечения jwt токена");
        // приватный ключ сервера для авторизации
        let server_encoding_key = env::var("SERVER_ENCODING_KEY")
            .expect("SERVER_ENCODING_KEY не найден в переменных окружения");

        // конфиг авторизации
        AuthConfig {
            jwt_expiry_time,
            server_encoding_key,
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self::new()
    }
}