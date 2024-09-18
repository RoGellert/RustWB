use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;

// структура для счётчика
#[derive(Debug)]
struct Counter {
    count: Mutex<i32>
}

// методы счётчика
impl Counter {
    // инициализация
    pub fn new() -> Self {
        Self {
            count: Mutex::new(0)
        }
    }

    // инкрементация
    pub fn increment(&self) {
        let mut guard = self.count.lock().unwrap();
        *guard+=1;
    }
}

#[tokio::main]
async fn main() {
    // инициализация структуры и вектора хэндлеров
    let counter = Arc::new(Counter::new());
    let mut handlers = vec![];

    // инициализация задач по инкрементации
    for _ in 0..4 {
        let counter_copy = Arc::clone(&counter);
        let handler = tokio::spawn(async move {
            for i in 1..=5 {

                counter_copy.increment();
                time::sleep(Duration::from_millis(200 * i)).await;
            }
        });

        handlers.push(handler);
    }

    // ожидание завершения задач
    for handler in handlers {
        handler.await.unwrap();
    }

    // вывод финальной стадии счётчика
    println!("Финальная стадия Counter {:?}", &counter);
}
