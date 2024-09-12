//! конфиг баз данных
use dotenv::dotenv;
use std::env;

// структура конфига базы данных
#[derive(Debug)]
pub struct DbConfig {
    pub pg_host: String,
    pub pg_user: String,
    pub pg_password: String,
    pub pg_dbname: String,
    pub redis_host: String,
    pub redis_port: String,
}

//
impl DbConfig {
    // Инициализация конфига и загрузка переменных окружения
    pub fn new() -> Self {
        // занрузка переменных окружения
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
        // хост базы данных Redis
        let redis_host =
            env::var("REDIS_HOST").expect("REDIS_HOST не найден в переменных окружения");
        // порт базы данных Postgres
        let redis_port =
            env::var("REDIS_PORT").expect("REDIS_PORT не найден в переменных окружения");

        DbConfig {
            pg_host,
            pg_user,
            pg_password,
            pg_dbname,
            redis_host,
            redis_port,
        }
    }
}

impl Default for DbConfig {
    fn default() -> Self {
        Self::new()
    }
}
