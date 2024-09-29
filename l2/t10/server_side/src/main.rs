use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                break;
            }
            Ok(n) => {
                // Читаем данные и выводим их на сервере
                let received_data = String::from_utf8_lossy(&buffer[..n]);
                println!("Received: {}", received_data);

                let response = format!("Echo: {}", received_data);
                stream.write_all(response.as_bytes()).expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                break;
            }
        }
    }
}

fn main() {
    // адрес для прослушивания на порту 8080
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Не удалось создать TcpListener на порту 8080");
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