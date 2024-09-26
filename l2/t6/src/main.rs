use std::collections::HashSet;
use std::env;
use std::io::{self};

///
/// пример: t6 -s -f 1-2,3 -d :
///
/// айди колонок в аргументе -f начинается с 1 (1-индексировано)
///

// преобразование строчки с колонками в вектор id колонок
fn parse_columns(columns_raw: String) -> Vec<usize> {
    // айдишники колонок без повторений
    let mut resulting_ids_hashed: HashSet<usize> = HashSet::new();
    // разделённые по запятой колонки с айди
    let separated_id_args: Vec<&str> = columns_raw.split(',').collect();

    // если строка из одного символа - запись одного айди, если 3 - проверка интревала и запись всех id из него
    for arg in separated_id_args {
        if arg.len() == 1 {
            resulting_ids_hashed
                .insert(arg.parse::<usize>().expect("неверный формат id колонки") - 1);
        } else if arg.len() == 3 {
            let split_range: Vec<&str> = arg.split('-').collect();
            let (start, finish): (usize, usize) = (
                split_range[0]
                    .parse::<usize>()
                    .expect("неверный формат id колонки"),
                split_range[1]
                    .parse::<usize>()
                    .expect("неверный формат id колонки"),
            );

            if start > finish {
                panic!("неверный аргумент с колонками")
            }

            for id in start..=finish {
                resulting_ids_hashed.insert(id - 1);
            }
        } else {
            panic!("неверный аргумент с колонками")
        }
    }

    // преобразование множества в вектор, а затем сортировка
    let mut resulting_ids: Vec<usize> = resulting_ids_hashed.into_iter().collect();
    resulting_ids.sort();

    resulting_ids
}

// запись в вектор и проверка входных агрументов
fn parse_args() -> (Vec<usize>, bool, char) {
    // возможные аргументы
    let possible_arguments: HashSet<&str> = HashSet::from(["-f", "-d", "-s"]);

    // текущие аргументы
    let mut command_arguments: HashSet<&str> = HashSet::new();

    // аргументы командной строки
    let args: Vec<String> = env::args().collect();

    // обработка аргументов
    if args.len() < 3 {
        panic!("недостаточное количество аргументов")
    }

    // требуемые колонки
    let mut columns_to_take: Vec<usize> = Vec::new();
    // брать ли колонки без раделителя
    let mut take_non_delim: bool = true;
    // разделитель
    let mut delimiter: char = '\t';

    // проверка и парсинг аргументов
    let mut i: usize = 1;
    while i < args.len() {
        let argument = &args[i];

        // если аргумент не присутсвует в сете возможных
        if !possible_arguments.contains(argument.as_str()) {
            panic!("неверный аргумент {:?}", argument);
        }

        // если аргумент повторный
        if command_arguments.contains(argument.as_str()) {
            panic!("повторный аргумент {:?}", argument);
        }

        // включение колонок без нужного разделителя
        if argument == "-s" {
            take_non_delim = false;
            command_arguments.insert(argument.as_str());
            i += 1;
            continue;
        }

        if argument == "-f" {
            // парсинг номера колонок в нужный формат
            let columns_str = args[i + 1].to_owned();
            columns_to_take = parse_columns(columns_str);
        } else if argument == "-d" {
            // запись разделителя в нужную форму
            delimiter = args[i + 1]
                .parse::<char>()
                .expect("разделитель должен быть одним символом");
        }

        i += 2;
        command_arguments.insert(argument.as_str());
    }

    (columns_to_take, take_non_delim, delimiter)
}

fn main() {
    // парсинг аргументов
    let (columns_to_take, take_non_delim, delimiter) = parse_args();

    // запись строки из std в вектор
    let mut lines: Vec<String> = io::stdin()
        .lines()
        .map(|line_res| line_res.unwrap())
        .collect();

    // удаление колонки без разделителя если нужно
    if !take_non_delim {
        lines.retain(|line| line.contains(delimiter));
    }

    // взятие нужных колонок из всех строчек и запись результата в массив строчек
    let mut vec_of_columns: Vec<Vec<String>> = Vec::new();
    for line in lines {
        // разделение колонок по разделителю
        let columns: Vec<&str> = line.split(delimiter).collect();

        // запись нужных колонок в массив
        let mut columns_temp: Vec<String> = Vec::new();
        for id in &columns_to_take {
            columns_temp.push(columns[*id].to_owned())
        }

        // запись нужных колонок в вектор со всеми строчками
        vec_of_columns.push(columns_temp)
    }

    // вывод нужых колонок с нужным разделителем в stdout
    for new_line in vec_of_columns {
        println!("{:?}", new_line.join(&delimiter.to_string()))
    }
}
