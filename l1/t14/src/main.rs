use std::any::{type_name_of_val};
use std::collections::HashSet;

fn main() {
    let a = vec![1, 2, 3];
    let b = 3.5;
    let c: HashSet<u64> = HashSet::new();

    println!("Тип переменной a: {}", type_name_of_val(&a));
    println!("Тип переменной b: {}", type_name_of_val(&b));
    println!("Тип переменной c: {}", type_name_of_val(&c));
}
