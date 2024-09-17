use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use dashmap::DashMap;
use tokio::time;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    // инициализация нужных объектов
    let hashmap_mutex = Arc::new(Mutex::new(HashMap::new()));
    let dash_map = Arc::new(DashMap::new());

    // создание 5 воркеров
    let mut handlers = vec![];
    for _ in 0..4 {
        // клонирование ARC в таски
        let hashmap_mutex_copy = Arc::clone(&hashmap_mutex);
        let dash_map_clone = Arc::clone(&dash_map);

        // начало работы вокреров
        let handler = tokio::spawn(async move {
            for i in 1..=5 {
                // добавление данных в структуры данных
                hashmap_mutex_copy.lock().unwrap().insert(i, i);
                dash_map_clone.insert(i, i);

                // вывод промежуточного состояния воркеров
                println!("Текущий Mutex c hashmap: {:?}", &hashmap_mutex_copy);
                println!("Текущий Dash map: {:?}", &dash_map_clone);

                // симуляция работы
                time::sleep(Duration::from_millis(200*i)).await;
            }
        });

        // добавление хэндлеров в массив
        handlers.push(handler);
    }

    // ожидание окончания работы воркеров
    for handler in handlers {
        handler.await.unwrap();
    }

    // вывод финального состояния воркеров
    println!("Финальный Mutex c hashmap: {:?}", &hashmap_mutex);
    println!("Финальный Dash map: {:?}", &dash_map);
}
