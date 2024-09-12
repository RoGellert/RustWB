//! декларация модулей для скриптов и декларация тестов
pub mod config;
pub mod db {
    pub mod postgres_db;
    pub mod redis_db;
}
pub mod controller;
pub mod model;

#[cfg(test)]
mod tests {
    use crate::model::Order;
    use reqwest::Client;
    use std::fs::File;
    use std::io::Read;

    #[tokio::test]
    // тест добавления и получения множества заказов из базы
    async fn test_add_many_orders() {
        let mut file = File::open("additional_files/model.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // десериализация прочтённых данных
        let mut orders: Vec<Order> = serde_json::from_str(&contents).unwrap();
        orders.truncate(6);

        // http post запросы с помощью reqwest
        let client = Client::new();
        for order in &orders {
            client
                .post("http://127.0.0.1:3000/orders")
                .json(order)
                .send()
                .await
                .unwrap();
        }

        // http get запрос с помощью reqwest
        let response = client
            .get("http://127.0.0.1:3000/orders")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        // десериализация в нужный struct
        let mut orders_from_request: Vec<Order> = serde_json::from_str(&response).unwrap();

        // сортировка для проверки
        orders_from_request.sort();
        orders.sort();

        // проверка на равенство
        assert_eq!(&orders_from_request, &orders)
    }

    #[tokio::test]
    // тест добавления и получения одного заказа из базы
    async fn test_add_one_order() {
        let mut file = File::open("additional_files/model.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // десериализация прочтённых данных
        let orders: Vec<Order> = serde_json::from_str(&contents).unwrap();
        let one_order: &Order = &orders[6];

        // http post запрос с помощью reqwest
        let client = Client::new();
        client
            .post("http://127.0.0.1:3000/orders")
            .json(&one_order)
            .send()
            .await
            .unwrap();

        // http get запрос с помощью reqwest
        let response = client
            .get(format!(
                "http://127.0.0.1:3000/orders/{}",
                &one_order.order_uid
            ))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        // десериализация в нужный struct
        let order_from_request: Order = serde_json::from_str(&response).unwrap();

        // проверка на равенство
        assert_eq!(&order_from_request, one_order)
    }
}

