fn reverse_string(string_to_reverse: &str) -> String {
    // итерация и переворачивание
    string_to_reverse.chars().rev().collect()
}

fn main() {
    // тест
    let sample_string = String::from("Thanks 😊");
    let reversed_string = reverse_string(&sample_string);

    println!("{}", &reversed_string);
}
