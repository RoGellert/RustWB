use tokio::time::Duration;
use tokio::{signal, time};
use tokio_util::sync::CancellationToken;

// вход в рантайм токио
#[tokio::main]
async fn main() {
    // токен для остановки трэдов
    let token = CancellationToken::new();

    // хэндлы тредов
    let mut handles = vec![];
    // старт нескольких воркеров
    for i in 0..5 {
        // данные для добавления в массив позже
        let mut curr = 1;
        let mut vec = vec![];

        // клон токена для остановки
        let cloned_token = token.clone();

        // начало рабты воркера
        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    // работа воркера
                    _ = time::sleep(Duration::from_secs(i+1)) => {
                        vec.push(curr);
                        curr += 1;
                        println!("Воркер {} работает", i);
                    }
                    // остановка воркера
                    _ = cloned_token.cancelled() => {
                        // тут должны быть запись в базу данных или другая форма сохранения
                        println!("Массив в воркере {} на момент остановки {:?} ", i, &vec);
                        println!("Воркер {} завершает работу", i);
                        break;
                    }
                }
            }
        });

        // добавления хэндлера в массив
        handles.push(handle);
    }

    // ожидания ввода и остановка воркеров
    signal::ctrl_c().await.expect("aaa");
    token.cancel();

    // ожидание окончания работы воркеров
    for worker in handles {
        worker.await.unwrap();
    }

    println!("Все воркеры завершили работу");
}
