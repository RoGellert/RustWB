//! инициализация и методы работы с базой данных Postgres
use crate::config::DbConfig;
use crate::model::{Delivery, Item, Order, Payment};
use deadpool_postgres::{
    Config as DeadpoolConfig, CreatePoolError, GenericClient, ManagerConfig, Pool, RecyclingMethod,
    Runtime,
};
use serde_json::Value;
use std::error::Error;
use tokio_postgres::NoTls;
use uuid::Uuid;

// обёртка вокруг пула подключений
pub struct PostgresDB {
    pool: Pool,
}

// парсинг данных окружения и создания конфига для deadpool
fn create_deadpool_config(db_config: &DbConfig) -> DeadpoolConfig {
    let mut cfg = DeadpoolConfig::new();
    cfg.dbname = Some((db_config.pg_dbname).to_string());
    cfg.user = Some((db_config.pg_user).to_string());
    cfg.password = Some((db_config.pg_password).to_string());
    cfg.host = Some((db_config.pg_host).to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg
}

// методы инициализации и работы с базой данных Postgres
impl PostgresDB {
    // создание инстанса базы данных опираясь на конфиг
    pub async fn new(db_config: &DbConfig) -> Result<Self, CreatePoolError> {
        // настройка конфига для подключения и пулинга
        let cfg = create_deadpool_config(db_config);

        // создание пула подключений
        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

        Ok(Self { pool })
    }

    // добавление нового заказа в базу
    pub async fn insert_order(&self, order: &Order) -> Result<(), Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        let statement = "
            INSERT INTO orders
            (order_uid,
            track_number,
            entry,
            locale,
            internal_signature,
            customer_id,
            delivery_service,
            shardkey,
            sm_id,
            date_created,
            oof_shard)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);
        ";

        // выполнение запроса с нужными данными
        client
            .query(
                statement,
                &[
                    &order.order_uid,
                    &order.track_number,
                    &order.entry,
                    &order.locale,
                    &order.internal_signature,
                    &order.customer_id,
                    &order.delivery_service,
                    &order.shardkey,
                    &order.sm_id,
                    &order.date_created,
                    &order.oof_shard,
                ],
            )
            .await?;

        // добавление новой доставки соответвующей заказу в базу
        self.insert_delivery(&order.delivery, &order.order_uid)
            .await?;
        // добавление новой оплаты соответвующей заказу в базу
        self.insert_payment(&order.payment, &order.order_uid)
            .await?;
        // добавление новых вещей, соответвующих заказу в базу
        self.insert_items(&order.items, &order.order_uid).await?;

        Ok(())
    }

    // функция для добавления доставки, относящейся к заказу, в базу
    async fn insert_delivery(
        &self,
        delivery: &Delivery,
        order_uid: &Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            INSERT INTO deliveries
            (name,
            phone,
            zip,
            city,
            address,
            region,
            email,
            order_uid)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8);
        ";

        // выполнение запроса с нужными данными
        client
            .query(
                statement,
                &[
                    &delivery.name,
                    &delivery.phone,
                    &delivery.zip,
                    &delivery.city,
                    &delivery.address,
                    &delivery.region,
                    &delivery.email,
                    order_uid,
                ],
            )
            .await?;

        Ok(())
    }

    // функция для добавления оплаты, относящейся к заказу, в базу
    async fn insert_payment(
        &self,
        payment: &Payment,
        order_uid: &Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            INSERT INTO payments
            (transaction,
            request_id,
            currency,
            provider,
            amount,
            payment_dt,
            bank,
            delivery_cost,
            goods_total,
            custom_fee,
            order_uid)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);
        ";

        // выполнение запроса с нужными данными
        client
            .query(
                statement,
                &[
                    &payment.transaction,
                    &payment.request_id,
                    &payment.currency,
                    &payment.provider,
                    &payment.amount,
                    &payment.payment_dt,
                    &payment.bank,
                    &payment.delivery_cost,
                    &payment.goods_total,
                    &payment.custom_fee,
                    order_uid,
                ],
            )
            .await?;

        Ok(())
    }

    // функция для добавления вещей, относящихся к заказу, в базу
    async fn insert_items(
        &self,
        items: &[Item],
        order_uid: &Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
            INSERT INTO items
            (chrt_id,
            track_number,
            price,
            rid,
            name,
            sale,
            size,
            total_price,
            nm_id,
            brand,
            status,
            order_uid)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12);
        ";

        // выполнение запроса с нужными данными
        for item in items.iter() {
            client
                .query(
                    statement,
                    &[
                        &item.chrt_id,
                        &item.track_number,
                        &item.price,
                        &item.rid,
                        &item.name,
                        &item.sale,
                        &item.size,
                        &item.total_price,
                        &item.nm_id,
                        &item.brand,
                        &item.status,
                        order_uid,
                    ],
                )
                .await?;
        }

        Ok(())
    }

    // функция для получения всех заказов из базы данных
    pub async fn get_all_orders(&self) -> Result<Option<Vec<Order>>, Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
                    SELECT json_agg(result) as order_json
                    FROM (
                        SELECT
                            orders.order_uid,
                            orders.track_number,
                            orders.entry,
                            orders.payment,
                            orders.locale,
                            orders.internal_signature,
                            orders.customer_id,
                            orders.delivery_service,
                            orders.shardkey,
                            orders.sm_id,
                            orders.date_created,
                            orders.oof_shard,
                            json_build_object(
                                'transaction', payments.transaction,
                                'request_id', payments.request_id,
                                'currency', payments.currency,
                                'provider', payments.provider,
                                'amount', payments.amount,
                                'payment_dt', payments.payment_dt,
                                'bank', payments.bank,
                                'delivery_cost', payments.delivery_cost,
                                'goods_total', payments.goods_total,
                                'custom_fee', payments.custom_fee
                            ) AS payment,
                            json_build_object(
                                'name', deliveries.name,
                                'phone', deliveries.phone,
                                'zip', deliveries.zip,
                                'city', deliveries.city,
                                'address', deliveries.address,
                                'region', deliveries.region,
                                'email', deliveries.email
                            ) as delivery,
                            json_agg(
                                json_build_object(
                                    'chrt_id', items.chrt_id,
                                    'track_number', items.track_number,
                                    'price', items.price,
                                    'rid', items.rid,
                                    'name', items.name,
                                    'sale', items.sale,
                                    'size', items.size,
                                    'total_price', items.total_price,
                                    'nm_id', items.nm_id,
                                    'brand', items.brand,
                                    'status', items.status
                                )
                            ) AS items
                        FROM
                            orders
                        INNER JOIN
                            payments ON orders.order_uid = payments.order_uid
                        INNER JOIN
                            deliveries ON orders.order_uid = deliveries.order_uid
                        INNER JOIN
                            items ON orders.order_uid = items.order_uid
                        GROUP BY
                            orders.order_uid, payments.payment_uid, deliveries.delivery_uid
                    ) result;
                ";

        // выполнение запроса
        let row = client.query_one(statement, &[]).await?;

        // парсинг json-а
        let orders_json_option: Option<Value> = row.get("order_json");

        // если json пуст, возврат Ok(None)
        let orders_json = match orders_json_option {
            None => return Ok(None),
            Some(orders_json) => orders_json,
        };

        // десериализация
        let orders: Vec<Order> = serde_json::from_value(orders_json)?;

        Ok(Some(orders))
    }

    // функция для получения одно заказа по uuid
    pub async fn get_one_order_by_uuid(
        &self,
        order_uid: &Uuid,
    ) -> Result<Option<Order>, Box<dyn Error + Send + Sync>> {
        // получение подключения из пула
        let client = self.pool.get().await?;

        // форма запроса
        let statement = "
                    SELECT json_agg(result) as order_json
                    FROM (
                        SELECT
                            orders.order_uid,
                            orders.track_number,
                            orders.entry,
                            orders.payment,
                            orders.locale,
                            orders.internal_signature,
                            orders.customer_id,
                            orders.delivery_service,
                            orders.shardkey,
                            orders.sm_id,
                            orders.date_created,
                            orders.oof_shard,
                            json_build_object(
                                'transaction', payments.transaction,
                                'request_id', payments.request_id,
                                'currency', payments.currency,
                                'provider', payments.provider,
                                'amount', payments.amount,
                                'payment_dt', payments.payment_dt,
                                'bank', payments.bank,
                                'delivery_cost', payments.delivery_cost,
                                'goods_total', payments.goods_total,
                                'custom_fee', payments.custom_fee
                            ) AS payment,
                            json_build_object(
                                'name', deliveries.name,
                                'phone', deliveries.phone,
                                'zip', deliveries.zip,
                                'city', deliveries.city,
                                'address', deliveries.address,
                                'region', deliveries.region,
                                'email', deliveries.email
                            ) as delivery,
                            json_agg(
                                json_build_object(
                                    'chrt_id', items.chrt_id,
                                    'track_number', items.track_number,
                                    'price', items.price,
                                    'rid', items.rid,
                                    'name', items.name,
                                    'sale', items.sale,
                                    'size', items.size,
                                    'total_price', items.total_price,
                                    'nm_id', items.nm_id,
                                    'brand', items.brand,
                                    'status', items.status
                                )
                            ) AS items
                        FROM
                            orders
                        INNER JOIN
                            payments ON orders.order_uid = payments.order_uid
                        INNER JOIN
                            deliveries ON orders.order_uid = deliveries.order_uid
                        INNER JOIN
                            items ON orders.order_uid = items.order_uid
                        WHERE orders.order_uid = $1
                        GROUP BY
                            orders.order_uid, payments.payment_uid, deliveries.delivery_uid
                    ) result;
                ";

        // выполнение запроса с нужными данными
        let row = client.query_one(statement, &[order_uid]).await?;

        // парсинг json-а
        let orders_json_option: Option<Value> = row.get("order_json");

        // если json пуст, возврат Ok(None)
        let orders_json = match orders_json_option {
            None => return Ok(None),
            Some(orders_json) => orders_json,
        };

        // десериализация
        let mut orders: Vec<Order> = serde_json::from_value(orders_json)?;
        let order: Order = orders.remove(0);

        Ok(Some(order))
    }
}
