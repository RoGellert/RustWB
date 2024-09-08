use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use l0::config::DbConfig;
use l0::model::OrdersModel;

#[tokio::main]
async fn main() {
    let db_config = DbConfig::new();
    let orders_model: Arc<OrdersModel> = Arc::new(OrdersModel::new(&db_config).await.unwrap());

    let mut file = File::open("json_files/model.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let orders: Vec<l0::model::Order> = serde_json::from_str(&contents).unwrap();

    for order in orders {
        orders_model.insert_order(&order).await.unwrap();
    }
}