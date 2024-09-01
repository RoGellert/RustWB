use std::io::Error;
use std::mem::forget;
use redis::{Commands, Connection, RedisError, RedisResult};

pub struct RedisDB {
    connection: Connection,
}

pub fn connect_to_redis(
    host: &str,
    port: &str
) -> Result<RedisDB, RedisError>{
    let client = redis::Client::open(format!("redis://{}:{}/", host, port))?;
    let connection = client.get_connection()?;

    let redis_instance = RedisDB { connection };

    Ok(redis_instance)
}

impl RedisDB {
    pub fn set(&mut self, key: &str, value: &str) -> RedisResult<()> {
        self.connection.set(key, value)?;
        Ok(())
    }

    pub fn get(&mut self, key: &str) -> RedisResult<Option<String>> {
        let data = self.connection.get(key)?;
        Ok(data)
    }

    pub fn del(&mut self, key: &str) -> RedisResult<()> {
        self.connection.del(key)?;
        Ok(())
    }
}
