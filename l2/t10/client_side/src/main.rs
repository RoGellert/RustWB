use std::env;
use std::io::{self, BufRead, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::thread;
use std::time::Duration;

///
/// серверная часть здесь:
///

fn main() {
    // получение аргументов командной строки
    let args: Vec<String> = env::args().collect();

    // хост
    let mut host: String;
    // порт
    let mut port: String;
    // тайм-аут, по умолчанию - две секунды
    let mut timeout: u64 = 2;

    // если аргумента 4
    if args.len() == 4 {
        // парсинг секунд тайм-аута
        if args[1].contains("--timeout") {
            let seconds_args: Vec<&str> = args[1].split('=').collect();
            let mut seconds_string = seconds_args[1].to_owned();
            seconds_string.pop();
            timeout = seconds_string
                .parse::<u64>()
                .expect("неверный формат количества количество секунд")
        } else {
            panic!("неверный формат воода секунд тайм-аута")
        }
        // парсинг хоста и порта
        host = args[2].to_owned();
        port = args[3].to_owned();
    // если аргумента 3
    } else if args.len() == 3 {
        // парсинг хоста и порта
        host = args[2].to_owned();
        port = args[3].to_owned();
    } else {
        panic!("неверное количество агрументов")
    }

    // преобразование в адрес
    let address = format!("{}:{}", host, port);
    let addr = address
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("не удалось преобразовать аргументы в адрес");

    // подключение с таймаутом
    let mut stream = TcpStream::connect_timeout(&addr, Duration::new(timeout, 0))
        .expect("невозможно подключиться к сокету");

    // установка тайм-аутов
    stream
        .set_read_timeout(Some(Duration::new(timeout, 0)))
        .expect("невозможно установить тайм-аут чтения в сокет");
    stream
        .set_write_timeout(Some(Duration::new(timeout, 0)))
        .expect("невозможно установить тайм-аут записывания в сокет");

    // клонирование потока для чтения в отдельном трэде
    let mut stream_clone = stream
        .try_clone()
        .expect("не удалось склонировать поток TCP");
    // создание отдельного потока для чтения
    thread::spawn(move || {
        let mut buffer = [0; 512];
        loop {
            match stream_clone.read(&mut buffer) {
                Ok(0) => {
                    println!("соеднинение закрыто сервером");
                    std::process::exit(0);
                }
                Ok(n) => {
                    println!(
                        "получено из сокета: {}",
                        String::from_utf8_lossy(&buffer[0..n])
                    );
                }
                Err(_) => {
                    panic!("хост разорвал подключение или ошибка чтения из сокета.");
                }
            }
        }
    });

    // основной цикл: чтение из STDIN и отправка в сокет
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("не удалось прочитать линию из std");
        stream
            .write_all(line.as_bytes())
            .expect("невозможно записать сообщение в сокет");
    }

    println!("закрытие подключения");
}
