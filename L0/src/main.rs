#[allow(unused_variables)]
mod modules {
    pub mod postgres_module;
    pub mod redis_module;
}

pub mod data_model;

#[tokio::main]
async fn main() {
    // let postgres_instance: PostgresDB =
    //     postgres_module::connect_to_db("localhost", "user", "password", "mydatabase")
    //         .await
    //         .unwrap();
    //
    // postgres_instance.drop_table("test").await.unwrap();

    // let mut redis_instance: RedisDB = redis_module::connect_to_redis("localhost", "6379").unwrap();
}
