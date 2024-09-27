use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str;
use std::sync::mpsc;
use std::time::Instant;
use std::{env, thread};

///
/// пример использования: t7 -f 5 input.txtt7
///

// структура для печати в формате json
#[derive(Serialize, Deserialize, Debug)]
struct Result {
    elapsed: String,
    result: HashMap<char, u32>,
}

// чтение из файла
fn read_lines_from_file(filename: &str) -> Vec<String> {
    let mut file_lines = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        file_lines.push(line.to_string())
    }
    file_lines
}

// считает символы из строки и записывает в HashMap
fn string_to_count(string: String) -> HashMap<char, u32> {
    let mut char_hash: HashMap<char, u32> = HashMap::new();
    for c in string.chars() {
        let char_ascii = c as u32;
        if (65..=90).contains(&char_ascii) || (97..=122).contains(&char_ascii) {
            match char_hash.get_mut(&c) {
                Some(elem) => {
                    *elem += 1;
                }
                None => {
                    char_hash.insert(c, 1);
                }
            };
        }
    }

    char_hash
}

// запись и проверка входных агрументов
fn parse_args() -> (usize, String) {
    // аргументы командной строки
    let args: Vec<String> = env::args().collect();

    // количество трэдов
    let mut threads_num: usize = 1;

    // парсинг в зависимости от числа аргументов
    let filename: String = match args.len() {
        2 => args[1].to_owned(),
        4 => {
            if args[1] != "-f" {
                panic!("неверный аргумент {:?}", args[1])
            };

            // парсинг номера трэдов
            threads_num = args[2]
                .parse::<usize>()
                .expect("неверный формат количества трэдов");

            args[3].to_owned()
        }
        _ => {
            panic!("неверное количесвто элементов")
        }
    };

    (threads_num, filename)
}

fn main() {
    // начальное время
    let now = Instant::now();

    // парсинг аргументов командной строки
    let (threads_num, filename) = parse_args();
    // каналы
    let (tx, rx) = mpsc::channel();

    // читает строки из файла и разбивает по трэдам насколько это возможно - не UTF символы не учитываются
    let string_of_chars: String = read_lines_from_file(&filename).join("");
    let elem_per_thread = string_of_chars.len().div_ceil(threads_num);
    let chunked_data: Vec<&str> = string_of_chars
        .as_bytes()
        .chunks(elem_per_thread)
        .map(str::from_utf8)
        .filter(|result| !result.is_err())
        .map(|result| result.unwrap())
        .collect();

    // массив трэдов для обработки
    let mut handles = vec![];

    // цикл для работы с частями данных
    for chunk in chunked_data {
        // передача владения частью данных
        let chunk_owned = chunk.to_owned();

        // клонирование канала отправки
        let tx_clone = tx.clone();

        // создания отдельного трэда для обработки
        let handle = thread::spawn(move || {
            // подсчёт чисел в каждом отдельном трэде
            let hash_set = string_to_count(chunk_owned);

            // отправка счёта в главный поток
            tx_clone.send(hash_set).unwrap();
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

    // обработка финальных данных (соединение всех hashMap из трэдов в единый объект)
    let mut final_result: HashMap<char, u32> = HashMap::new();
    rx.iter().for_each(|map| {
        for (key, value) in map {
            match final_result.get_mut(&key) {
                Some(elem) => {
                    *elem += value;
                }
                None => {
                    final_result.insert(key, value);
                }
            };
        }
    });

    // конечное время
    let time_elapsed_string = format!("elapsed: {} ms", now.elapsed().as_millis());

    // создание объекта результата для финального вывода
    let result = Result {
        elapsed: time_elapsed_string,
        result: final_result,
    };

    // печать финальных данных
    println!("{:?}", serde_json::to_string(&result).unwrap());
}
