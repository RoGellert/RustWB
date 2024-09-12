use dotenv::dotenv;
use log::error;
use std::env;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    // загрузка данных из окружения
    dotenv().ok();

    // парсинг переменных окружения
    let pg_host = env::var("PG_HOST").expect("PG_HOST не найден в переменных окружения");
    let pg_user = env::var("PG_USER").expect("PG_USER не найден в переменных окружения");
    let pg_password =
        env::var("PG_PASSWORD").expect("PG_PASSWORD не найден в переменных окружения");
    let pg_dbname = env::var("PG_DBNAME").expect("PG_DBNAME не найден в переменных окружения");

    // разовое подключение к базе данных
    let (client, connection) = tokio_postgres::connect(
        &format!(
            "host={} user={} password={} dbname={}",
            pg_host, pg_user, pg_password, pg_dbname
        ),
        NoTls,
    )
    .await
    .unwrap();

    // инициализация подключения
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {e}");
        }
    });

    // запросы

    client.query("DROP TABLE deliveries;", &[]).await.unwrap();

    client.query("DROP TABLE payments;", &[]).await.unwrap();

    client.query("DROP TABLE items;", &[]).await.unwrap();

    client.query("DROP TABLE orders;", &[]).await.unwrap();

    client
        .query(r#"DROP EXTENSION "uuid-ossp";"#, &[])
        .await
        .unwrap();
}
