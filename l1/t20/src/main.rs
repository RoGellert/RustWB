// нужная форма интерфейса в форме trait
trait Printer {
    fn print(&self, msg: &str);
}

// несовместимая структура
struct IncompatiblePrinter;
// адаптер
struct AdapterPrinter {
    incompatible_printer: IncompatiblePrinter
}

impl IncompatiblePrinter {
    // инициализания
    fn new() -> Self {
        Self
    }

    // несовместимая функция
    fn print_incompatible(&self, msg: &str) {
        println!("{}", msg)
    }
}

impl AdapterPrinter {
    // инициализания
    fn new(incompatible_printer: IncompatiblePrinter) -> Self {
        Self {
            incompatible_printer
        }
    }
}

impl Printer for AdapterPrinter {
    // совместимая функция
    fn print(&self, msg: &str) {
        self.incompatible_printer.print_incompatible(msg);
    }
}

fn main() {
    // тест
    let incompatible_printer = IncompatiblePrinter::new();
    let adapter_printer = AdapterPrinter::new(incompatible_printer);

    adapter_printer.print("Сообщение")
}
