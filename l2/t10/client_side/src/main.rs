use std::env;
use std::io::{self, BufRead, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::thread;

fn main() {
    // получение аргументов командной строки
    let args: Vec<String> = env::args().collect();

    // хост
    let mut host: String = String::new();
    // порт
    let mut port: String = String::new();
    // тайм-аут, по умолчанию - две секунды
    let mut timeout: u64 = 2;

    // если аргумента 4
    if args.len() == 4 {
        if args[1].contains("--timeout") {
            let seconds_string: Vec<&str> = args[1].split('=').collect();
            timeout = seconds_string[1][..seconds_string[1].len()].parse::<u64>().unwrap();
        }
        host = args[2].to_owned();
        port = args[3].to_owned();
    } else if args.len() == 3 {
        host = args[2].to_owned();
        port = args[3].to_owned();
    } else {
        panic!("неверное количество агрументов")
    }

    // преобразование в адрес
    let address = format!("{}:{}", host, port);
    let addr = address.to_socket_addrs().unwrap().next().unwrap();

    // подключение с таймаутом
    let mut stream = TcpStream::connect_timeout(&addr, Duration::new(timeout, 0)).expect("невозможно подключиться к сокету");

    // установка тайм-аутов
    stream
        .set_read_timeout(Some(Duration::new(timeout, 0)))
        .expect("невозможно ");
    stream
        .set_write_timeout(Some(Duration::new(timeout, 0)))
        .expect("Failed to set write timeout");

    // клонирование потока для чтения в отдельном трэде
    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");
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
                    println!("получено из сокета: {}", String::from_utf8_lossy(&buffer[0..n]));
                }
                Err(e) => {
                    eprintln!("ошибка чтения из сокета: {}", e);
                    std::process::exit(1);
                }
            }
        }
    });

    // Основной цикл: чтение из STDIN и отправка в сокет
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line from stdin");
        match stream.write_all(line.as_bytes()) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Failed to write to socket: {}", e);
                break;
            }
        }
    }

    println!("Closing connection.");
}
