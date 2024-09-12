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

        // ожидание окончания работы трэда
        handle.join().unwrap()
    }
}
