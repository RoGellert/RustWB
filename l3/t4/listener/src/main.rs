use crate::config::Config;
use crate::kafka_producer::KafkaProducer;

mod config;
mod kafka_producer;

fn main() {
    let config = Config::new();

    let kafka_producer = KafkaProducer::new(&config.kafka_config);

    println!("всё ок")
}
