use std::thread;

fn main() {
    // количество чисел в массиве
    let n = 100;
    // создание массива
    let vec: Vec<u32> = (0..n).collect();

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

        // создания отдельного трэда для печати
        let handle = thread::spawn(move || {
            for num in chunk_owned {
                println!("{}", num*num);
            }
        });

        // добавления трэда в массив трэдов
        handles.push(handle);
    }

    // ожидание окончания работы трэдов
    for handle in handles {
        handle.join().unwrap();
    }
}
