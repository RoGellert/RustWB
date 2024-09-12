use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use clap::Parser;
use rand::Rng;

///  Для запуска
///   cargo run -- --read-worker-num [число воркеров]
///  Например
///   cargo run -- --read-worker-num 5


// аргументы командной строки
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // количество воркеров для чтения
    #[arg(short, long, default_value_t = 1)]
    read_worker_num: u32,
}

fn main() {
    // парсинг аргументов командной строки
    let args = Args::parse();
    let read_worker_num = args.read_worker_num;

    // создание массива для добавления данных
    let data_to_read: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(vec![]));

    // создания воркеров
    for i in 0..read_worker_num {
        // клонирования ссылки на данные
        let data_clone = Arc::clone(&data_to_read);

        // создание воркера
        thread::spawn(move || {
            // создание объекта для генеерации случайных чисел
            let mut rng = rand::thread_rng();

            // бесконечный цикл
            loop {
                // блокирование данных через Mutex для беопасности потоков
                let vec = data_clone.lock().unwrap();

                // если массив не пуст, вывод номера воркера и случайного элемента из массива
                if !vec.is_empty() {
                    let random_index = rng.gen_range(0..vec.len());
                    println!("Значение: {}, воркер: {}", &vec[random_index], &i + 1);
                }
                else {
                    println!("Массив пуст")
                }

                // деаллокация и разблокировка Mutex
                drop(vec);

                // сон со случайной продолжительностью
                thread::sleep(Duration::from_millis(rng.gen_range(2000..10000)));
            }
        });
    }

    // счётчик для удобства
    let mut counter = 1;
    loop {
        // блокирование данных через Mutex для беопасности потоков
        let mut vec = data_to_read.lock().unwrap();

        // добавление нового элемента в массив
        let data_to_add = counter;
        vec.push(data_to_add);
        println!("В массив добавлено значение: {}", &counter);

        // инкерементация счётчика
        counter += 1;

        // деаллокация и разблокировка Mutex
        drop(vec);

        // сон
        thread::sleep(Duration::from_millis(1000));
    }
}
