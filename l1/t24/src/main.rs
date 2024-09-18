use std::collections::HashSet;

fn has_unique_chars(string: &str) -> bool {
    // капитальные буквы в lowercase
    let string_lower = string.to_lowercase();

    // создания множества присутствующих в строке букв для удаления дубликатов
    let string_set: HashSet<char> = string_lower.chars().collect();

    // если длина изначальной строки такая же как и длина множества - значит все элементы уникальны
    string_set.len() == string_lower.len()
}

fn main() {
    // тест
    let str1 = "abcd";
    let str2 = "abCdefAaf";
    let str3 = "aabcd";

    println!(
        "Уникальные ли символы в строке: {}? Ответ: {}",
        str1,
        has_unique_chars(str1)
    );
    println!(
        "Уникальные ли символы в строке: {}? Ответ: {}",
        str2,
        has_unique_chars(str2)
    );
    println!(
        "Уникальные ли символы в строке: {}? Ответ: {}",
        str3,
        has_unique_chars(str3)
    );
}
