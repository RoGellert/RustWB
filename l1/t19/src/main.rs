fn reverse_words(words: &str) -> String {
    // новая строка
    let mut string_to_return = String::new();
    // итерация через слова и получение слов в обратном аорядке
    for word in words.split(' ').rev() {
        string_to_return.push_str(&format!("{} ", word));
    }
    // удаление последнего " "
    string_to_return.pop();

    string_to_return
}

fn main() {
    let words_to_reverse = "Hello world";
    let words_reversed = reverse_words(words_to_reverse);

    println!("{:?}", &words_reversed);
}
