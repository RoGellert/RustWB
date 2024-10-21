//! конфиг баз данных
use dotenv::dotenv;
use std::env;

// структура конфига базы данных
#[derive(Clone)]
pub struct DbConfig {
    pub pg_host: String,
    pub pg_user: String,
    pub pg_password: String,
    pub pg_dbname: String,
}

impl DbConfig {
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
        DbConfig {
            pg_host,
            pg_user,
            pg_password,
            pg_dbname,
        }
    }
}

impl Default for DbConfig {
    fn default() -> Self {
        Self::new()
    }
}
