mod data_model;

#[allow(unused_variables)]
mod db_modules {
    pub mod postgres_module;
    pub mod redis_module;
}

use db_modules::postgres_module::PostgresDB;
use db_modules::postgres_module;

#[tokio::main]
async fn main() {
    let postgres_instance: PostgresDB =
        postgres_module::connect_to_postgres("localhost", "user", "password", "mydatabase")
            .await
            .unwrap();

    postgres_instance.drop_table("test").await.unwrap();

    //let mut redis_instance: RedisDB = redis_module::connect_to_redis("localhost", "6379").unwrap();
}
