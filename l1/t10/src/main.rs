use std::time::Duration;
use tokio::time;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    // создание каналов
    let (tx_num, rx_num) = flume::unbounded::<i32>();
    let (tx_num_sqr, rx_num_sqr) = flume::unbounded::<i32>();

    // создание токенов для отмены
    let token = CancellationToken::new();
    let token_clone_send_num = token.clone();
    let token_clone_send_square = token.clone();
    let token_clone_receive = token.clone();

    // создание и запуск задачи по отправке
    let mut curr = 1;
    let send_num_handler = tokio::spawn(async move {
        loop {
            tokio::select! {
                // отправка числа
                _ = time::sleep(Duration::from_secs(1)) => {
                   tx_num.send(curr).unwrap();
                   curr += 1
                }
                // отключение по токену
                _ = token_clone_send_num.cancelled() => {
                    println!("Остановка таска отправки чисел по токену");
                    break;
                }
            }
        }
    });

    // создание и запуск таска по принятию чисел и отправке квадратов чисел
    let send_square_num_handler = tokio::spawn(async move {
        loop {
            tokio::select! {
                // принятие числа и отправка квадрата
                _ = time::sleep(Duration::from_secs(1)) => {
                   let data = rx_num.recv().unwrap();
                   tx_num_sqr.send(data.pow(2)).unwrap();
                }
                // отключение по токену
                _ = token_clone_send_square.cancelled() => {
                    println!("Остановка таска отправки чисел во второй степени по токену");
                    break;
                }
            }
        }
    });

    // создание и запуск таска по принятию квадратов чисел
    let receive_num_handler = tokio::spawn(async move {
        let mut data_vec = vec![];
        loop {
            tokio::select! {
                // принятие числа
                _ = time::sleep(Duration::from_secs(1)) => {
                   let data = rx_num_sqr.recv().unwrap();
                   data_vec.push(data);
                   println!("Текущее состояние данных с квадратами чисел: {:?} ", &data_vec);
                }
                // отключение по токену
                _ = token_clone_receive.cancelled() => {
                    println!("Остановка таска отправки чисел во второй степени по токену");
                    println!("Финальное состояние данных с квадратами чисел: {:?} ", &data_vec);
                    break;
                }
            }
        }
    });

    // через 10 секунд - завершение
    time::sleep(Duration::from_secs(10)).await;
    token.cancel();

    // ожидание завершения всех тасок
    send_num_handler.await.unwrap();
    send_square_num_handler.await.unwrap();
    receive_num_handler.await.unwrap();
}
