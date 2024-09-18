fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    // левый и правый индекс
    let mut left = 0;
    let mut right = arr.len() - 1;

    // если правый меньше левого - элемент не найден
    while left <= right {
        // середина массива
        let middle = left + (right - left) / 2;

        if target == &arr[middle] {
            // элемент найден
            return Some(middle)
        } else if target > &arr[middle] {
            // поиск в правой части если элемен больше элемента по середине
            left = middle + 1;
        } else {
            // в противном случае поиск в левой части
            right = middle - 1
        }
    }

    None
}

fn main() {
    // тест
    let arr = [1, 2, 3, 4, 5, 6, 7];
    println!("Индекс элемента 4: {:?}", &binary_search(&arr, &4));
    println!("Индекс элемента 8: {:?}", &binary_search(&arr, &8));
}
