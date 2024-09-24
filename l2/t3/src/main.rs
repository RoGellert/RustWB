use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{read_to_string, File};
use std::io::Write;

/// Колонки в аргументах 1-индексированные =)
///
/// Имплементированные аргументы: "-k", "-n", "-r", "-u", "-M", "-c"
///
/// Пример использования: t3.exe -M -k 3 -c -r input.txt output


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

// записывает отсортированные строки из вектора в файл
fn write_lines_to_file(filename: &str, lines: Vec<String>) {
    let mut file = File::create(format!("{}.txt", filename)).expect("не удалось создать файл");

    for line in &lines {
        writeln!(file, "{}", line).expect("ошибка при записи строки в файл");
    }
}

// сортировка по числовому значению
fn sort_by_numbers(
    lines_to_sort: &mut [String],
    column_to_sort_by: usize,
    reversed: bool,
    check_sorted: bool,
) {
    // проверка отсортированы ли данные
    if check_sorted
        && lines_to_sort.windows(2).all(|pair| {
        let a_columns: Vec<&str> = pair[0].split_whitespace().collect();
        let b_columns: Vec<&str> = pair[1].split_whitespace().collect();

        let num1 = a_columns[column_to_sort_by].parse::<i32>()
            .expect("невозможно конвертировать текст в колонке в число");
        let num2 = b_columns[column_to_sort_by].parse::<i32>()
            .expect("невозможно конвертировать текст в колонке в число");

        if reversed {
            num1 >= num2
        } else {
            num1 <= num2
        }
    })
    {
        println!("колонки уже отсортированы по числовому значению");
        return;
    };

    // сортировка по числовому значению
    lines_to_sort.sort_by(|a, b| {
        let a_columns: Vec<&str> = a.split_whitespace().collect();
        let b_columns: Vec<&str> = b.split_whitespace().collect();

        let num1 = a_columns[column_to_sort_by].parse::<i32>()
            .expect("невозможно конвертировать текст в колонке в число");
        let num2 = b_columns[column_to_sort_by].parse::<i32>()
            .expect("невозможно конвертировать текст в колонке в число");

        num1.cmp(&num2)
    });

    // если нужна сортировка в обратном порядке
    if reversed {
        lines_to_sort.reverse()
    }
}

// сортировка по тексту
fn sort_by_text(
    lines_to_sort: &mut [String],
    column_to_sort_by: usize,
    reversed: bool,
    check_sorted: bool,
) {
    // проверка отсортированы ли данные
    if check_sorted
        && lines_to_sort.windows(2).all(|pair| {
            let a_columns: Vec<&str> = pair[0].split_whitespace().collect();
            let b_columns: Vec<&str> = pair[1].split_whitespace().collect();

            if reversed {
                a_columns[column_to_sort_by] >= b_columns[column_to_sort_by]
            } else {
                a_columns[column_to_sort_by] <= b_columns[column_to_sort_by]
            }
        })
    {
        println!("колонки уже отсортированы по тексту");
        return;
    };

    // сортировка по месяцам
    lines_to_sort.sort_by(|a, b| {
        let a_columns: Vec<&str> = a.split_whitespace().collect();
        let b_columns: Vec<&str> = b.split_whitespace().collect();

        a_columns[column_to_sort_by].cmp(b_columns[column_to_sort_by])
    });

    // если нужна сортировка в обратном порядке
    if reversed {
        lines_to_sort.reverse()
    }
}

// сортировка по месяцам
fn sort_by_month(
    lines_to_sort: &mut [String],
    column_to_sort_by: usize,
    reversed: bool,
    check_sorted: bool,
) {
    // возможные месяцы
    let possible_months: HashMap<&str, usize> = HashMap::from([
        ("JAN", 1),
        ("FEB", 2),
        ("MAR", 3),
        ("APR", 4),
        ("MAY", 5),
        ("JUN", 6),
        ("JUL", 7),
        ("AUG", 8),
        ("SEP", 9),
        ("OCT", 10),
        ("NOV", 11),
        ("DEC", 12),
    ]);

    // проверка отсортированы ли данные
    if check_sorted
        && lines_to_sort.windows(2).all(|pair| {
            let a_columns: Vec<&str> = pair[0].split_whitespace().collect();
            let b_columns: Vec<&str> = pair[1].split_whitespace().collect();

            let mapped_month_one = possible_months
                .get(a_columns[column_to_sort_by])
                .expect("неверный формат месяца");
            let mapped_month_two = possible_months
                .get(b_columns[column_to_sort_by])
                .expect("неверный формат месяца");

            if reversed {
                mapped_month_one >= mapped_month_two
            } else {
                mapped_month_one <= mapped_month_two
            }
        })
    {
        println!("колонки уже отсортированы по месяцам");
        return;
    };

    // сортировка по месяцам
    lines_to_sort.sort_by(|a, b| {
        let a_columns: Vec<&str> = a.split_whitespace().collect();
        let b_columns: Vec<&str> = b.split_whitespace().collect();

        let mapped_month_one = possible_months
            .get(a_columns[column_to_sort_by])
            .expect("неверный формат месяца");
        let mapped_month_two = possible_months
            .get(b_columns[column_to_sort_by])
            .expect("неверный формат месяца");

        mapped_month_one.cmp(mapped_month_two)
    });

    // если нужна сортировка в обратном порядке
    if reversed {
        lines_to_sort.reverse()
    }
}

// записывает в вектор и проверяет входные агрументы
fn parse_args() -> (usize, String, String, String, bool, bool) {
    // возможные аргументы
    let possible_arguments: HashSet<&str> =
        HashSet::from(["-k", "-n", "-r", "-u", "-M", "-c"]);

    // аргументы командной строки
    let args: Vec<String> = env::args().collect();

    // обработка аргументов
    if args.len() < 3 {
        panic!("недостаточное количество аргументов")
    }

    // имя входного файла (предпоследний аргумент)
    let output_file_name = args.last().unwrap().to_owned();
    // имя выходного файла (последний аргумент)
    let input_file_name = args[args.len() - 2].to_owned();

    // массив для обработанных аргументов
    let mut command_arguments = HashSet::new();
    // колонка по которой сортировать > 0
    let mut column_to_sort_by: usize = 0;
    // в каком порядке сортировать
    let mut reversed: bool = false;
    // проверять отсортирован ли ввод
    let mut check_sorted: bool = false;
    // по каком типу сортировать
    let mut mode: String = String::new();

    // проверка и парсинг аргументов
    let mut i = 1;
    while i < args.len() - 2 {
        // если аргумент не присутсвует в сете возможных
        if !possible_arguments.contains(args[i].as_str()) {
            panic!("неверный аргумент {:?}", args[i]);
        }

        // если аргумент повторный
        if command_arguments.contains(args[i].as_str()) {
            panic!("повторный аргумент {:?}", args[i]);
        }

        // аргумент колонки для сортировки
        if args[i] == "-k" {
            column_to_sort_by = args[i + 1]
                .parse::<usize>()
                .expect("неверный номер колонки для сортировки");
            i += 2;
            continue;
        }

        // возиожные варианты сортировки - по умолчанию - по текстовому значению
        if args[i] == "-M" || args[i] == "-n" {
            if mode.is_empty() {
                mode = args[i].to_owned();
            } else {
                panic!("несколько способов сортировки задано в аргументах")
            }
        }

        // навправление сортировки
        if args[i] == "-r" {
            reversed = true;
        }

        // проверять ли отсортированы ли строчки
        if args[i] == "-c" {
            check_sorted = true;
        }

        command_arguments.insert(args[i].as_str());
        i += 1
    }

    // если не была задана колонка для сортировки
    if column_to_sort_by == 0 {
        panic!(
            "не задана или неверна колонка для сортировки: {}",
            column_to_sort_by
        )
    }

    // -1 в колонке в связи с 0-индексированием =)
    (
        column_to_sort_by - 1,
        input_file_name,
        output_file_name,
        mode,
        reversed,
        check_sorted,
    )
}

fn main() {
    // чтение аргументов командной строки
    let (
        column_to_sort_by,
        input_file_name,
        output_file_name,
        mode,
        reversed,
        check_sorted,
    ) = parse_args();
    // чтение колонок для сортировки
    let mut lines_to_sort = read_lines_from_file(&input_file_name);

    // сортировка в зависимости от вида
    match mode.as_str()
    {
        "-n" => sort_by_numbers(
            &mut lines_to_sort,
            column_to_sort_by,
            reversed,
            check_sorted,
        ),
        "-M" => sort_by_month(
            &mut lines_to_sort,
            column_to_sort_by,
            reversed,
            check_sorted,
        ),
        _ => sort_by_text(
            &mut lines_to_sort,
            column_to_sort_by,
            reversed,
            check_sorted,
        )
    }

    // запись в файл
    write_lines_to_file(&output_file_name, lines_to_sort);
}
