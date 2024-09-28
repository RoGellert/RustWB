use std::env;
use std::fs::File;
use std::io::Write;
use std::error::Error;

///
/// Пример использования: t9 https://github.com downloaded_page
///
/// Результат будет в файле: downloaded_page.html
///

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // аргументы командной строки
    let args: Vec<String> = env::args().collect();

    // проверка количества аргументов
    if args.len() != 3 {
        panic!("Неверное количество аргументов: требуется ввести url сайта как аргумент")
    }

    // достать url из аргументов
    let url = args[1].to_owned();
    // достать url из аргументов
    let result_file_name = args[2].to_owned();

    // загрузить сайт
    let response = reqwest::get(url).await.expect("не удалось загрузить сайн");

    // обработать текст
    let content = response.text().await.expect("не удалось обработать текст");

    // создать файл и записать в него содержимое сайта
    let mut file = File::create(format!("{}.html", result_file_name))?;
    file.write_all(content.as_bytes())?;

    Ok(())
}