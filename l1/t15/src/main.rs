fn quicksort<T: Ord>(arr: &mut [T]) {
    // если в оставшейся части всего 1 елемент - его нет сымсла сортировать
    if arr.len() <= 1 {
        return;
    }

    // индекс, в котором оказался pivot в результате работы алгоритма
    // по факту является финальным место элемента в отсортированном массиве
    let pivot_index = partition(arr);

    // рекурсивный вызов алгоритма на оставшиеся части
    quicksort(&mut arr[..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    // инициализация правого индекса
    let mut i = 0;
    // выбор самого правого элемента как pivot
    let pivot_index = arr.len() - 1;

    // перетасовка элементов с помощью вспомогательного индекса j
    for j in 0..pivot_index {
        if arr[j] < arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }
    // перестановка pivot-элеманта на правильное место
    arr.swap(i, arr.len() - 1);
    i
}

fn main() {
    // тест
    let mut arr = [1, 5, 15, 10 , 4, 12 , 9, 2];

    println!("Массив до быстрой сортировки: {:?}", arr);
    quicksort(&mut arr);
    println!("Массив после быстрой сортировки: {:?}", arr);
}
