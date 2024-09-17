use std::io;
use std::io::BufRead;

fn main() {
    // строка для записи повторений
    let mut repeats = String::new();

    // стандартный ввод
    let stdin = io::stdin();

    // для каждой линии в стандартнов вводе
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        // если ввод пуст, конец цикла
        if line.is_empty() {
            break
        }

        // если ввод не повторяется с прошлого раза, вывод строку в стандартный вывод
        if line != repeats {
            println!("{}", &line);
            repeats = line;
        }
    }
}