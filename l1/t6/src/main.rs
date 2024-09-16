use flume::RecvError;
use tokio::time;
use tokio::time::Duration;
use tokio_util::sync::CancellationToken;

// вход в рантайм токио
#[tokio::main]
async fn main() {
    // каналы для отпраки и получения
    let (tx, rx) = flume::unbounded();

    // токен для остановки трэдов
    let token = CancellationToken::new();

    // данные для добавления в массив позже
    let mut curr = 1;

    // клон для канала отправки
    let cloned_token_send = token.clone();

    // начало рабты воркера для отправки
    let send_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
            // работа воркера
            _ = time::sleep(Duration::from_secs(1)) => {
                tx.send_async(curr).await.unwrap();
                curr += 1;
            }
            // остановка воркера при отмене токена
            _ = cloned_token_send.cancelled() => {
                    println!("Воркер для отправки завершает работу");
                    drop(tx);
                    break;
                }
            }
        }
    });

    // данные для хранения воркером получения
    let mut data = vec![];

    // клон токена для канала получения
    let cloned_token_recv = token.clone();

    // начало рабты воркера для получения
    let receive_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
            // работа воркера
            _ = time::sleep(Duration::from_millis(3)) => {
                let received_data_result = rx.recv_async().await;

                // если канал отправки отключен - остановка воркера
                let received_data = match received_data_result {
                    Ok(res)  => res,
                    Err(RecvError::Disconnected) => {
                        println!("Канал по отправке отключен, отключенение канала по получению");
                        break;
                    }
                };

                // вывод и запись данных
                data.push(received_data);
                println!("Текущие данные: {:?}", &data);
            }
            // остановка воркера при отмене токена
            _ = cloned_token_recv.cancelled() => {
                    println!("Воркер для получения завершает работу");
                    break;
                }
            }
        }
    });

    // n секунд работы
    let n_seconds = 10;
    time::sleep(Duration::from_secs(n_seconds)).await;

    // отмена токена
    token.cancel();

    // ожидание завершения работы воркеров
    send_handle.await.unwrap();
    receive_handle.await.unwrap();

    println!("Все воркеры завершили работу");
}
