use std::sync::mpsc;
use std::thread;

fn main() {
    // количество чисел в массиве
    let n = 50;
    // создание массива
    let vec: Vec<u32> = (0..n).collect();

    // каналы
    let (tx, rx) = mpsc::channel();

    // количество элементов для одного трэда
    let chunk_size = 10;
    // вектор из ссылок на оригинальный массив
    let chunked_data: Vec<&[u32]> = vec.chunks(chunk_size).collect();

    // массив трэдов для обработки
    let mut handles = vec![];

    // цикл для работы с частями данных
    for chunk in chunked_data {
        // передача владения частью данных
        let chunk_owned = chunk.to_owned();

        // клонирование канала отправки
        let tx_clone = tx.clone();

        // создания отдельного трэда для печати
        let handle = thread::spawn(move || {
            // локальная для трэда сумма квадратов
            let mut sum_of_squares: u32 = 0;
            for num in chunk_owned {
                sum_of_squares += num * num;
            }

            // отправка суммы квадратов в канал получения
            tx_clone.send(sum_of_squares).unwrap();
        });

        // добавления хэндлера в массив
        handles.push(handle);
    }

    // ожидание окончания работы трэдов
    for handle in handles {
        handle.join().unwrap();
    }

    // удаление оригинального канала отправки
    drop(tx);

    // обработка финальных данных
    let sum_of_squares: u32 = rx.iter().sum();
    println!("{}", sum_of_squares);
}