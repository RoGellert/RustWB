use std::thread;
use std::thread::JoinHandle;
use flume::{Receiver, TryRecvError};
use tokio::time;
use tokio::time::Duration;
use tokio_util::sync::CancellationToken;

// функция для создания задачи токио с остановкой на cancellation token, сообщение и отключения канала
fn spawn_tokio_handler(cancellation_token: CancellationToken, receiver: Receiver<()>, num: i32) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = time::sleep(Duration::from_secs(1)) => {
                    // получение сообщений / информации о сбросе канала
                    let received_message_result = receiver.try_recv();

                    // остановка работы
                    match received_message_result {
                        Ok(())  => {
                            println!("Остановка задачи Tokio {} по сообщению", num);
                            break;
                        },
                        Err(TryRecvError::Disconnected) => {
                            println!("Канал по отправке отключен, отключенение воркера tokio {}", num);
                            break;
                        },
                        Err(_) => {
                            println!("Воркер tokio {} работает", num);
                        }
                    };
                }
                // отключение по токену
                _ = cancellation_token.cancelled() => {
                    println!("Остановка задачи Tokio {} по токену", num);
                    break;
                }
            }
        }
    })
}

// функция для создания трэда с отключением чераз сообщение/сброс канала
fn spawn_thread_handler(receiver: Receiver<()>, num: i32) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            // получение сообщений / информации о сбросе канала
            let received_result = receiver.try_recv();

            // остановка работы
            match received_result {
                Ok(())  => {
                    println!("Остановка треда {} по сообщению", num);
                    break;
                },
                Err(TryRecvError::Disconnected) => {
                    println!("Канал по отправке отключен, отключенение трэда {}", num);
                    break;
                },
                Err(_) => {}
            };

            // в ротивном случае работа
            println!("Воркер thread {} работает", num);

            thread::sleep(Duration::from_millis(500));
        }
    })
}

// вход в рантайм токио
#[tokio::main]
async fn main() {
    // каналы для отправки и получения
    let (tx_thread_1, rx_thread_1) = flume::unbounded::<()>();
    let (tx_thread_2, rx_thread_2) = flume::unbounded::<()>();
    let (_tx_tokio_1, rx_tokio_1) = flume::unbounded::<()>();
    let (tx_tokio_2, rx_tokio_2) = flume::unbounded::<()>();
    let (tx_tokio_3, rx_tokio_3) = flume::unbounded::<()>();

    // токены для остановки трэдов tokio
    let token_tokio_1 = CancellationToken::new();
    let token_tokio_2 = CancellationToken::new();
    let token_tokio_3 = CancellationToken::new();

    // инициализация трэдов и tokio
    let handler_tokio_1 = spawn_tokio_handler(token_tokio_1.clone(), rx_tokio_1, 1);
    let handler_tokio_2 = spawn_tokio_handler(token_tokio_2.clone(), rx_tokio_2, 2);
    let handler_tokio_3 = spawn_tokio_handler(token_tokio_3.clone(), rx_tokio_3,3);
    let handler_thread_1 = spawn_thread_handler(rx_thread_1, 1);
    let handler_thread_2 = spawn_thread_handler(rx_thread_2, 2);

    // разнообразные способы закрытия каналов
    time::sleep(Duration::from_secs(2)).await;
    token_tokio_1.cancel();

    time::sleep(Duration::from_secs(2)).await;
    tx_tokio_2.send(()).unwrap();

    time::sleep(Duration::from_secs(2)).await;
    drop(tx_tokio_3);

    time::sleep(Duration::from_secs(2)).await;
    tx_thread_1.send(()).unwrap();

    time::sleep(Duration::from_secs(2)).await;
    drop(tx_thread_2);

    // завершение работы
    handler_tokio_1.await.unwrap();
    handler_tokio_2.await.unwrap();
    handler_tokio_3.await.unwrap();
    handler_thread_1.join().unwrap();
    handler_thread_2.join().unwrap();

    println!("Thread и tokio завершили работу")
}