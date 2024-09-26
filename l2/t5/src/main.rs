use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

///
/// Пример: t5 -i -c -C 1  helloWorld input.txt
///
/// -v работает с -A -B -C не выводя N строк до/после/вокруг
///

// записывает строки из файла в вектор
fn read_lines_from_file(filename: &str) -> Vec<String> {
    let mut file_lines = Vec::new();
    for line in read_to_string(filename)
        .expect("не удалось записать строку из файла в вектор")
        .lines()
    {
        file_lines.push(line.to_string())
    }
    file_lines
}

// находит совпадающие с regex строки и возвращает их индекс в HashMap
fn get_regex_match_ids(
    input_strings: &Vec<String>,
    regex_pattern: String,
    register_ignore: bool,
) -> HashSet<usize> {
    // массив для совпадающих айди
    let mut matching_ids: HashSet<usize> = HashSet::new();

    // если нужно игнорировать регистр
    let regex_opinionated = match register_ignore {
        true => regex_pattern.to_lowercase(),
        false => regex_pattern,
    };

    // создание объекта regex
    let re = Regex::new(&regex_opinionated).unwrap();

    // запись id совпадений
    for (i, line) in input_strings.iter().enumerate() {
        // если нужно игнорировать регистр
        let line_opinionated = match register_ignore {
            true => line.to_lowercase(),
            false => line.to_owned(),
        };

        // есть ли совпадение
        if re.is_match(&line_opinionated) {
            matching_ids.insert(i);
        }
    }

    matching_ids
}

// находит совпадающие с фиксированной строкой строки из записывает их индекс
fn get_fixed_match_ids(
    input_strings: &Vec<String>,
    regex_pattern: String,
    register_ignore: bool,
) -> HashSet<usize> {
    // массив для совпадающих айди
    let mut matching_ids: HashSet<usize> = HashSet::new();

    // если нужно игнорировать регистр
    let regex_opinionated = match register_ignore {
        true => regex_pattern.to_lowercase(),
        false => regex_pattern.to_owned(),
    };

    // запись id совпадений
    for (i, line) in input_strings.iter().enumerate() {
        // если нужно игнорировать регистр
        let line_opinionated = match register_ignore {
            true => line.to_lowercase(),
            false => line.to_owned(),
        };

        // есть ли совпадение
        if line_opinionated == regex_opinionated {
            matching_ids.insert(i);
        }
    }

    matching_ids
}

// возвращяет HashMap с id с N строками до и изначальными id
fn get_required_ids_before(
    ids_raw: HashSet<usize>,
    adjacent_lines_num: usize,
) -> HashSet<usize> {
    let mut ids_processed: HashSet<usize> = HashSet::new();

    for id in ids_raw {
        let low = match id < adjacent_lines_num {
            false => id - adjacent_lines_num,
            true => 0
        };

        for id_new in low..=id {
            ids_processed.insert(id_new);
        }
    }

    ids_processed
}

// возвращяет HashMap с id с N строками после и изначальными id
fn get_required_ids_after(
    ids_raw: HashSet<usize>,
    max_id: usize,
    adjacent_lines_num: usize,
) -> HashSet<usize> {
    let mut ids_processed: HashSet<usize> = HashSet::new();

    for id in ids_raw {
        let high = (id + adjacent_lines_num).min(max_id);

        for id_new in id..=high {
            ids_processed.insert(id_new);
        }
    }

    ids_processed
}

// возвращяет HashMap с id с N строками вокруг и изначальными id
fn get_required_ids_around(
    ids_raw: HashSet<usize>,
    max_id: usize,
    adjacent_lines_num: usize,
) -> HashSet<usize> {
    let mut ids_processed: HashSet<usize> = HashSet::new();

    for id in ids_raw {
        let low = match id < adjacent_lines_num {
            false => id - adjacent_lines_num,
            true => 0
        };
        let high = (id + adjacent_lines_num).min(max_id);

        for id_new in low..=high {
            ids_processed.insert(id_new);
        }
    }

    ids_processed
}

fn get_matches(
    input_strings: Vec<String>,
    regex_pattern: String,
    mode: String,
    invert: bool,
    fixed: bool,
    register_ignore: bool,
    adj_mode: String,
    adjacent_lines_num: usize,
) {
    // количество входных строк
    let input_len = input_strings.len();

    // HashSet с id совпавших строк
    let matched_ids: HashSet<usize> = match fixed {
        true => get_fixed_match_ids(&input_strings, regex_pattern, register_ignore),
        false => get_regex_match_ids(&input_strings, regex_pattern, register_ignore),
    };

    // HashSet с id совпавших и соседних строк - в зависимости от аргумента
    let mut processed_ids = match adj_mode.as_str() {
        "-A" => get_required_ids_after(matched_ids, input_len-1, adjacent_lines_num),
        "-B" => get_required_ids_before(matched_ids, adjacent_lines_num),
        "-C" => get_required_ids_around(matched_ids, input_len-1, adjacent_lines_num),
        _ => matched_ids,
    };

    // HashSet исключая id совпавших и соседних строк - в зависимости от аргумента
    if invert {
        let all_ids: HashSet<usize> = (0..input_len).collect();
        processed_ids = &all_ids - &processed_ids;
    }

    // вывод нужой информации в зависимости от входного аргумента
    match mode.as_str() {
        // печать количества строк
        "-c" => {
            println!("{}", processed_ids.len());
        },
        // печать id
        "-n" => {
            for i in 0..input_len {
                if processed_ids.contains(&i) {
                    println!("{}", i);
                }
            }
        },
        // печать строк
        _ => {
            for (i, line) in input_strings.iter().enumerate() {
                if processed_ids.contains(&i) {
                    println!("{}", line);
                }
            }
        }
    }
}

// записывает в вектор и проверяет входные агрументы
fn parse_args() -> (String, bool, bool, bool, String, String, String, usize) {
    // возможные аргументы
    let possible_arguments: HashSet<&str> =
        HashSet::from(["-A", "-B", "-C", "-c", "-i", "-v", "-F", "-n"]);

    // возможные типы -grep
    let possible_types: HashSet<&str> = HashSet::from(["-c", "-n"]);

    // возможные виды вывода соседних строк
    let possible_adj_types: HashSet<&str> = HashSet::from(["-A", "-B", "-C"]);

    // аргументы командной строки
    let args: Vec<String> = env::args().collect();

    // обработка аргументов
    if args.len() < 3 {
        panic!("недостаточное количество аргументов")
    }

    // имя входного файла (предпоследний аргумент)
    let input_file_name = args.last().unwrap().to_owned();
    // regex - паттерн
    let regex_string = args[args.len() - 2].to_owned();

    // массив для обработанных аргументов
    let mut command_arguments = HashSet::new();
    // как выводить соседние линии
    let mut adj_mode: String = String::new();
    // по каком типу сортировать
    let mut mode: String = String::new();
    // инвертирована ли команда
    let mut invert: bool = false;
    // точное ли совпадение
    let mut fixed: bool = false;
    // игнорировать ли регистр
    let mut register_ignore: bool = false;
    // количество линий для вывода
    let mut adjacent_lines_num: usize = 0;

    let mut i: usize = 1;
    // проверка и парсинг аргументов
    while i < args.len() - 2 {
        let argument = &args[i];

        // если аргумент не присутсвует в сете возможных
        if !possible_arguments.contains(argument.as_str()) {
            panic!("неверный аргумент {:?}", argument);
        }

        // если аргумент повторный
        if command_arguments.contains(argument.as_str()) {
            panic!("повторный аргумент {:?}", argument);
        }

        // как выводить соседние линии
        if possible_adj_types.contains(argument.as_str()) {
            if adj_mode.is_empty() {
                adjacent_lines_num = args[i + 1]
                    .parse::<usize>()
                    .expect("неверное число соседних строк для вывода");

                adj_mode = argument.to_owned();
                command_arguments.insert(argument.as_str());
                i += 2;
                continue;
            } else {
                panic!("несколько способов вывода соседних линий")
            }
        }
        // возможные варианты выведения строк
        else if possible_types.contains(argument.as_str()) {
            if mode.is_empty() {
                mode = argument.to_owned();
            } else {
                panic!("несколько способов сортировки задано в аргументах")
            }
        }
        // инвертирована ли команда
        else if argument == "-v" {
            invert = true;
        }
        // проверять отсортированы ли строчки
        else if argument == "-F" {
            fixed = true;
        }
        // игнорировать ли регистр
        else if argument == "-i" {
            register_ignore = true;
        }

        command_arguments.insert(argument.as_str());

        i += 1
    }

    (
        mode,
        invert,
        fixed,
        register_ignore,
        input_file_name,
        regex_string,
        adj_mode,
        adjacent_lines_num,
    )
}

fn main() {
    // запарсить аргументы
    let (
        mode,
        invert,
        fixed,
        register_ignore,
        input_file_name,
        regex_string,
        adj_mode,
        adjacent_lines_num,
    ) = parse_args();
    // чтение колонок для матчинга
    let lines_to_match = read_lines_from_file(&input_file_name);

    // получение совпадений и вывод в стд
    get_matches(
        lines_to_match,
        regex_string,
        mode,
        invert,
        fixed,
        register_ignore,
        adj_mode,
        adjacent_lines_num,
    );
}
