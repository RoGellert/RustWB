/// для тестов - cargo test

pub fn unpack(input: &str) -> String {
    // стэк для обрабоки символов
    let mut stack: Vec<char> = vec![];
    // строчка для финального результата
    let mut result = String::new();

    // цикл проходящий по всем элементам
    for c in input.chars() {
        // если символ является числом
        if c.is_numeric() {
            let char_to_append: char;

            // если символ в стэке один
            if stack.len() == 1 {
                // если в стэке: один escape - символ добавляет числовой символ в стэк
                if stack[0] == '\\' {
                    stack.push(c);
                    continue;
                }

                // в противном случае записывает этот элемент для последующего дублирования
                char_to_append = stack[0];
            } else if stack.len() == 2 && stack[0] == '\\' {
                // если элеметов в стэке два и первый: escape - символ; записыват второй элемент для дублирования
                char_to_append = stack[1];
            } else {
                // в остальных случаях паникует
                panic!("некорректная строка")
            }

            // дублирует нужный символ
            let number = c.to_digit(10).unwrap();
            let chars_to_push: String = (0..number).map(|_| char_to_append).collect();
            // добавляет в строку
            result.push_str(&chars_to_push);
            // и очищает stack
            stack.clear();

            continue;
        }

        // если символ является escape - символом
        if c == '\\' {
            if stack.is_empty() {
                // если стэк пуст, добавляет символ
                stack.push(c);
            } else if stack.len() == 1 {
                // если первый символ в стэке - escape, добавляет ещё один в стэк
                if stack[0] == '\\' {
                    stack.push(c);
                    continue;
                }

                // иначе записывает в финальную строку символ из стэка
                result.push(stack[0]);
                // чистит стэк
                stack.clear();
                // и записывает escape символ в стэк
                stack.push(c);
            } else if stack.len() == 2 && stack[0] == '\\' {
                // если символов в стэке 2 и первый: escape, добавляет в финальную строку 2 элемент
                result.push(stack[1]);
                // чистит стэк
                stack.clear();
                // и добавляет escape в стэк
                stack.push(c);
            } else {
                // в противном случае паникует
                panic!("некорректная строка")
            }

            continue;
        }

        // для всех остальных символов
        if stack.is_empty() {
            // если в стэке ничего нет - записывает в стэк
            stack.push(c);
        } else if stack.len() == 1 {
            // если первый символ в стэке - escape; записывает символ в стэк
            if stack[0] == '\\' {
                stack.push(c);
            } else {
                // в остальных случаях с одним элементов записывает символ в финальную строку
                // элемент из стэка
                result.push(stack[0]);
                // очищает стэк
                stack.clear();
                // и записывает новый симво в стэк
                stack.push(c);
            }
        }
    }

    // после прохода цикла, если в стэке остались элементы
    if stack.len() == 1 {
        if stack[0] == '\\' || stack[0].is_numeric() {
            // если элемент один и он - escape или число: паникует
            panic!("некорректная строка")
        } else {
            // в противном случае записывает элемент в финальную строку
            result.push(stack[0])
        }
    } else if stack.len() == 2 {
        // если элементов в стеке два и первый - escape;
        // добавляет в финальную строку второй символ из стека
        if stack[0] == '\\' {
            result.push(stack[1])
        } else {
            // иначе паникует
            panic!("некорректная строка")
        }
    } else if !stack.is_empty() {
        // в остальных случаях если стэк не пустой - паникует
        panic!("некорректная строка")
    }

    result
}

// тесты
#[cfg(test)]
mod tests {
    use crate::unpack;

    #[test]
    fn test_str1() {
        assert_eq!(unpack("a4bc2d5e"), "aaaabccddddde".to_string());
    }

    #[test]
    fn test_str2() {
        assert_eq!(unpack("abcd"), "abcd".to_string());
    }

    #[test]
    #[should_panic]
    fn test_str3() {
        unpack("45");
    }

    #[test]
    fn test_str4() {
        assert_eq!(unpack(""), "".to_string());
    }

    #[test]
    fn test_str5() {
        assert_eq!(unpack("qwe\\4\\5"), "qwe45".to_string());
    }

    #[test]
    fn test_str6() {
        assert_eq!(unpack("qwe\\45"), "qwe44444".to_string());
    }

    #[test]
    fn test_str7() {
        assert_eq!(unpack("qwe\\\\5"), "qwe\\\\\\\\\\".to_string());
    }
}

fn main() {
    // еще один тест =)
    println!("{}", unpack("qwe\\\\5"));
}