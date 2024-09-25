use std::collections::{HashMap, HashSet};

// функция подсчета букв в слове
fn word_to_count(word: &str) -> [i32; 33] {
    let mut letter_count = [0; 33];

    for c in word.chars() {
        letter_count[(c.to_ascii_lowercase() as u32 - 'а'.to_ascii_lowercase() as u32) as usize] += 1;
    }

    letter_count
}

// функция создания нужной HashMap
fn create_anagram_map(input_words: &Vec<&str>) -> HashMap<String, Vec<String>> {
    // промежуточкая HashMap для группировки анаграмм
    let mut anagram_map: HashMap<[i32; 33], HashSet<String>> = HashMap::new();
    // промежуточкая HashMap для учёта первого встретившегося слова
    let mut first_word_map: HashMap<[i32; 33], String> = HashMap::new();

    // цикл, который считает количество определлёных букв в слове и хэшируя массив со счётом группирует анаграммы
    for word in input_words {
        let word_lowercase = word.to_lowercase();
        let count = word_to_count(&word_lowercase);

        match anagram_map.get_mut(&count) {
            Some(value) => { value.insert(word_lowercase); },
            None => {
                first_word_map.insert(count, word_lowercase.clone());
                anagram_map.insert(count, HashSet::from([word_lowercase]));
            }
        }
    }

    // удаляет из HashMap векторы из одного элемента
    anagram_map.retain(|_, v| v.len() > 1);

    // преобразует данные из двух HashMap в финальную форму, при этом сортируя массивы
    let mut resulting_map: HashMap<String, Vec<String>> = HashMap::new();
    for (k, v) in anagram_map {
        let mut word_vec: Vec<String> = v.into_iter().collect();
        word_vec.sort_by(|a, b| {
            a.cmp(b)
        });

        resulting_map.insert(first_word_map[&k].to_owned(), word_vec);
    };

    resulting_map
}

fn main() {
    // тест
    let input_words= vec!["АБВГДКК", "Пятак", "тяпка", "листок", "слиток", "столик", "пятка"];

    let resulting_map = create_anagram_map(&input_words);

    println!("{:?}", resulting_map);
}