//! конфиг баз данных
use dotenv::dotenv;
use std::env;

// структура конфига
#[derive(Clone)]
pub struct Config {
    pub db_config: DbConfig,
    pub kafka_config: KafkaConfig
}

// структура конфига базы данных
#[derive(Clone)]
pub struct DbConfig {
    pub pg_host: String,
    pub pg_user: String,
    pub pg_password: String,
    pub pg_dbname: String,
}

// структура конфига сервиса кафка
#[derive(Clone)]
pub struct KafkaConfig {
    pub kafka_port: String,
    pub kafka_host: String,
}

impl Config {
    // инициализация конфига и загрузка переменных окружения
    pub fn new() -> Self {
        // загрузка переменных окружения
        dotenv().ok();

        // хост базы данных Postgres
        let pg_host = env::var("PG_HOST").expect("PG_HOST не найден в переменных окружения");
        // юзер базы данных Postgres
        let pg_user = env::var("PG_USER").expect("PG_USER не найден в переменных окружения");
        // пароль базы данных Postgres
        let pg_password =
            env::var("PG_PASSWORD").expect("PG_PASSWORD не найден в переменных окружения");
        // имя базы данных Postgres
        let pg_dbname = env::var("PG_DBNAME").expect("PG_DBNAME не найден в переменных окружения");

        // конфиг базы данных
        let db_config = DbConfig {
            pg_host,
            pg_user,
            pg_password,
            pg_dbname,
        };

        // хост сервиса Kafka
        let kafka_host = env::var("KAFKA_HOST").expect("KAFKA_HOST не найден в переменных окружения");
        // порт сервиса Kafka
        let kafka_port = env::var("KAFKA_PORT").expect("KAFKA_PORT не найден в переменных окружения");

        // конфиг базы данных
        let kafka_config = KafkaConfig {
            kafka_port,
            kafka_host
        };

        Config {
            db_config,
            kafka_config
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
