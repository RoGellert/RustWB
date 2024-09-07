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

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {e}");
        }
    });

    client
        .query(r#"CREATE EXTENSION IF NOT EXISTS "uuid-ossp";"#, &[])
        .await
        .unwrap();

    client
        .query(
            "CREATE TABLE orders (
                        order_uid UUID PRIMARY KEY,
                        track_number VARCHAR,
                        entry VARCHAR,
                        payment VARCHAR,
                        locale VARCHAR,
                        internal_signature VARCHAR,
                        customer_id VARCHAR,
                        delivery_service VARCHAR,
                        shardkey VARCHAR,
                        sm_id integer,
                        date_created TIMESTAMP,
                        oof_shard VARCHAR
                    );",
            &[],
        )
        .await
        .unwrap();

    client
        .query(
            "CREATE TABLE deliveries (
                    delivery_uid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                    order_uid UUID UNIQUE REFERENCES orders(order_uid),
                    name VARCHAR,
                    phone VARCHAR,
                    zip VARCHAR,
                    city VARCHAR,
                    address VARCHAR,
                    region VARCHAR,
                    email VARCHAR
                );",
            &[],
        )
        .await
        .unwrap();

    client
        .query(
            "CREATE TABLE payments (
                     payment_uid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                     transaction VARCHAR,
                     order_uid UUID UNIQUE REFERENCES orders(order_uid),
                     request_id VARCHAR,
                     currency VARCHAR,
                     provider VARCHAR,
                     amount integer,
                     payment_dt integer,
                     bank VARCHAR,
                     delivery_cost integer,
                     goods_total integer,
                     custom_fee integer
                );",
            &[],
        )
        .await
        .unwrap();

    client
        .query(
            "CREATE TABLE items (
                    item_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                    order_uid UUID REFERENCES orders(order_uid),
                    chrt_id integer,
                    track_number VARCHAR,
                    price integer,
                    rid VARCHAR,
                    name VARCHAR,
                    sale integer,
                    size VARCHAR,
                    total_price integer,
                    nm_id integer,
                    brand VARCHAR,
                    status integer
                );",
            &[],
        )
        .await
        .unwrap();
}
