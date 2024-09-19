use std::env;
use std::fs::read_to_string;

// записывает строки из файла в вектор
fn read_lines_from_file(filename: &str) -> Vec<String> {
    let mut file_lines = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        file_lines.push(line.to_string())
    }
    file_lines
}

// считает количество строк
fn count_lines(vector_of_lines: &Vec<String>) -> usize {
    vector_of_lines.len()
}

// считает количество слов
fn count_words(vector_of_lines: &Vec<String>) -> usize {
    let mut counter: usize = 0;
    for line in vector_of_lines {
        let word_vec: Vec<&str> = line.split(' ').collect();
        counter += word_vec.len();
    }
    counter
}

// считает количество символов включая пробелы
fn count_symbols(vector_of_lines: &Vec<String>) -> usize {
    let mut counter: usize = 0;
    for line in vector_of_lines {
        counter += line.len();
    }
    counter
}

fn main() {
    // читает аргументы из командной строки
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // если аргумента 2 - читает строки и выводит количество слов
        2 => {
            let file_lines = read_lines_from_file(&args[1]);
            println!("{}", count_words(&file_lines));
        },
        // если аргумента 3 - проверяет аргументы и выполняет нужную команду
        3 => {
            let file_lines = read_lines_from_file(&args[2]);
            match args[1].as_str() {
                // выводит количество символов, включая whitespace (пробел)
                "-c" => {
                    println!("{}", count_symbols(&file_lines))
                },
                // выводит количество строк
                "-l" => {
                    println!("{}", count_lines(&file_lines))
                },
                // выводит количество слов
                "-w" => {
                    println!("{}", count_words(&file_lines))
                },
                // паникует если аргумент неправильный
                _ => {
                    panic!("Допустимые аргументы: -c -l -w")
                }
            }
        },
        _ => {
            panic!("Неверное количество агрументов")
        }
    }
}
