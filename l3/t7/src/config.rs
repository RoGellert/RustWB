//! конфиг баз данных
use dotenv::dotenv;
use std::env;

// структура конфига базы данных
#[derive(Debug)]
pub struct RedisConfig {
    pub redis_host: String,
    pub redis_port: String,
}

impl RedisConfig {
    // Инициализация конфига и загрузка переменных окружения
    pub fn new() -> Self {
        // загрузка переменных окружения
        dotenv().ok();

        // хост базы данных Redis
        let redis_host =
            env::var("REDIS_HOST").expect("REDIS_HOST не найден в переменных окружения");
        // порт базы данных Postgres
        let redis_port =
            env::var("REDIS_PORT").expect("REDIS_PORT не найден в переменных окружения");

        RedisConfig {
            redis_host,
            redis_port,
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self::new()
    }
}
