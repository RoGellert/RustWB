use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// обработка подключений
fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("клиент отключился");
                break;
            }
            Ok(n) => {
                // чтение данных и запись в сокет
                let received_data = String::from_utf8_lossy(&buffer[..n]);
                println!("новые данные записаны в сокет: {}", received_data);

                let response = received_data.to_owned();
                stream.write_all(response.as_bytes()).expect("не удалость записать данные в сокет");
            }
            Err(e) => {
                eprintln!("не удалость прочитать данные из сокета: {}", e);
                break;
            }
        }
    }
}

fn main() {
    // адрес для прослушивания на порту 8080
    let listener = TcpListener::bind("0.0.0.0:8080").expect("не удалось создать TcpListener на порту 8080");
    println!("сервер готов принимать соединения через сокет на порту 8080");

    // принятие подключений в вечном цикле
    for tcp_stream in listener.incoming() {
        match tcp_stream {
            Ok(stream) => {
                // параллельная обработка запросов
                thread::spawn(move || {
                    handle_request(stream);
                });
            }
            Err(e) => {
                eprintln!("не удалось подключится к клиенту: {:?}", e)
            }
        }
    }
}