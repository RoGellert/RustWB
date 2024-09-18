fn main() {
    let mut vector = vec![1, 2, 3, 4, 5];

    println!("Вектор до удаления {:?}", &vector);
    vector.remove(2);
    println!("Вектор после удаления {:?}", &vector);
}
