//! конфиг баз данных
use dotenv::dotenv;
use std::env;

// структура конфига базы данных
pub struct DbConfig {
    pub pg_host: String,
    pub pg_user: String,
    pub pg_password: String,
    pub pg_dbname: String,
}

// структура конфига авторизации
pub struct AuthConfig {
    pub jwt_expiry_time: u32,
    pub server_encoding_key: String,
}

// струкутра общего конфига
pub struct Config {
    pub db_config: DbConfig,
    pub auth_config: AuthConfig,
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

        // срок истечения токена JWT в секундах
        let jwt_expiry_time = env::var("JWT_EXPIRY_TIME")
            .expect("JWT_EXPIRY_TIME не найден в переменных окружения")
            .parse::<u32>()
            .expect("неверное количество секунд истечения jwt токена");
        // приватный ключ сервера для авторизации
        let server_encoding_key = env::var("SERVER_ENCODING_KEY")
            .expect("SERVER_ENCODING_KEY не найден в переменных окружения");

        // конфиг авторизации
        let auth_config = AuthConfig {
            jwt_expiry_time,
            server_encoding_key,
        };

        // общий конфиг
        Config {
            db_config,
            auth_config,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
