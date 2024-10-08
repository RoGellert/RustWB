/// вывод:
/// 1
/// 4
/// 5
/// 6
/// wrap 8
/// 8
/// 0
/// 3
/// 2

// структура-пример с одним объектом типа i32
struct Example(i32);

// trait с функцией drop (деструктор) для структуры Example который вызывается при сбросе
impl Drop for Example {
    // печатает поле из структуры при сбросе
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

// структура-обёртка с одним объектом типа Example
struct ExampleWrap(Example);

// trait с функцией drop (деструктор) для структуры-обёртки ExampleWrap который вызывается при сбросе
impl Drop for ExampleWrap {
    fn drop(&mut self) {
        // перемещает старый Example в переменную e, заменяет предыдущий Example на новый в памяти
        let e = std::mem::replace(&mut self.0, Example(0));
        // печатает поле из старого Example
        println!("wrap {}", e.0);

        // сбрасывает старый Example из стэка вызывая деструктор
        // сбрасывает новый Example из стэка вызывая деструктор
    }
}

fn main() {
    // временный объект без имени - сразу же вызывает функцию drop и сбрасывается из памяти
    Example(1);

    // объект с именем - последний на сброс в стэке (при сбросе вызывает функцию drop и сбрасывается из памяти)
    let _e2 = Example(2);

    // объект с именем - предпоследний на сброс в стэке (при сбросе вызывает функцию drop и сбрасывается из памяти)
    let _e3 = Example(3);

    // временный объект без имени - сразу же вызывает функцию drop и сбрасывается из памяти
    let _ = Example(4);

    // объявление неинициализированной переменной
    let mut _e5;

    // инициализация enum Option со структурой Example внутри
    _e5 = Some(Example(5));

    // присваивается поле None к enum Option - сбрасывается структрура Example внутри и вызывается метод drop
    _e5 = None;

    // объект с именем e6 - сбрасывается в следущей строчке
    let e6 = Example(6);
    // явный сброс e6 с вызовом функции drop
    drop(e6);

    // объект с именем e7 - сбрасывается в следущей строчке без вызова деструктора
    let e7 = Example(7);

    // сброс объекта e7 из памяти без вызова деструктора
    std::mem::forget(e7);

    // создания объекта - обертки с новым объектом примера внутри - комментарии в декларации трэйта Drop
    ExampleWrap(Example(8));

    // сбрасывается ExampleWrap(Example(8)) с вызовом деструктора
    // сбрасывается Example(3) с вызовом деструктора
    // сбрасывается Example(2) с вызовом деструктора
}
