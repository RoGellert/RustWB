#[allow(unused_variables)]
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String
}

#[derive(Debug, Deserialize, Serialize)]
struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    provider: String,
    amount: u32,
    payment_dt: u32,
    bank: String,
    delivery_cost: u32,
    goods_total: u32,
    custom_fee: u32
}

#[derive(Debug, Deserialize, Serialize)]
struct Item {
    chrt_id: u32,
    track_number: String,
    price: u32,
    rid: String,
    name: String,
    sale: u32,
    size: String,
    total_price: u32,
    nm_id: u32,
    brand: String,
    status: u32
}

#[derive(Debug, Deserialize, Serialize)]
struct Order {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: u32,
    date_created: String,
    oof_shard: String,
}

// fn read_and_deserialize<P: AsRef<Path>>(path: P) -> Result<Vec<Order>, Box<dyn Error>> {
//
// }

fn main() {
    let mut file = File::open("files/model.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let order: Order = serde_json::from_str(&data).unwrap();

    println!("orders : {:#?}", order);
}
