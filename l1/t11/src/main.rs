use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::vec;

fn main() {
    // вектор температур
    let temps: Vec<f64> = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];

    // новая HashMap для записи темперптур в интервалы
    let mut hash_map: HashMap<i64, Vec<f64>> = HashMap::new();

    // для каждой температуры - запись в HashMap,
    // используя как ключ нижную часть интервала к которому принадлежит температура
    for temp in temps {
        let temp_floor = ((temp / 10_f64) as i64) * 10;
        match hash_map.entry(temp_floor) {
            Entry::Vacant(e) => {
                e.insert(vec![temp]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(temp);
            }
        }
    }

    // сортировка по интерваламы и конвертация в массив
    let mut vec = vec![];
    for key in hash_map.clone().into_keys().sorted() {
        let Some(temp_vec) = hash_map.get(&key) else {
            panic!("aaaa")
        };
        if key < 0 {
            vec.push(((key - 10, key), temp_vec.to_owned()))
        } else {
            vec.push(((key, key + 10), temp_vec.to_owned()))
        }
    }

    println!("{:?}", vec)
}
