use std::collections::HashSet;

fn get_intersection(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    // новое множество для возврата
    let mut new_set = HashSet::new();

    // небольшая оптимизация
    let (shorter_set, longer_set) = if set1.len() > set2.len() {
        (set2, set1)
    } else {
        (set1, set2)
    };

    // сравнение множеств
    for elem in shorter_set.iter() {
        if longer_set.get(elem).is_some() {
            new_set.insert(*elem);
        }
    }

    new_set
}

fn main() {
    // тест
    let set1 = HashSet::from([1, 2, 3, 4, 5]);
    let set2 = HashSet::from([1, 4, 5]);

    // своя функция
    println!("Пересечение: {:?}", &get_intersection(&set1, &set2));
    // функция библиотеки
    println!("Пересечение: {:?}", &set1.intersection(&set2));
}
