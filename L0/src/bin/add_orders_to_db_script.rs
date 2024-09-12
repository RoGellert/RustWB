//! скрипт для добавления данных в базу через API
use l0::model::Order;
use reqwest::Client;
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() {
    // чтение данных для добавления из файлика json
    let mut file = File::open("additional_files/model.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // десериализация прочтённых данных
    let orders: Vec<Order> = serde_json::from_str(&contents).unwrap();

    // http post запросы с помощью reqwest
    let client = Client::new();
    for order in orders {
        client
            .post("http://127.0.0.1:3000/orders")
            .json(&order)
            .send()
            .await
            .unwrap();
    }
}
