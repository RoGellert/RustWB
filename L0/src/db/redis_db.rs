// use crate::config::DbConfig;
// use std::sync::{Arc, Mutex};
// use redis::{AsyncCommands, Client, RedisResult};
//
// pub struct RedisDB {
//     client: Client,
// }
//
// impl RedisDB {
//     pub fn new(db_config: &DbConfig) -> RedisResult<RedisDB> {
//         let client = Client::open(format!("redis://{}:{}/", db_config.redis_host, db_config.redis_port))?;
//
//         Ok(Self {
//             client,
//         })
//     }

// pub fn set(&mut self, key: &str, value: &str) -> RedisResult<()> {
//     let con =
//     self.connection.set(key, value)?;
//     Ok(())
// }
//
// pub fn get(&mut self, key: &str) -> RedisResult<Option<String>> {
//     let data = self.connection.get(key)?;
//     Ok(data)
// }
//
// pub fn del(&mut self, key: &str) -> RedisResult<()> {
//     self.connection.del(key)?;
//     Ok(())
// }
// }
