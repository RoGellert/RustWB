use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;
use tracing::{error, info, Level};
use notify::{recommended_watcher, RecursiveMode, Result, Watcher};
use notify::event::CreateKind::Any;
use notify::EventKind::{Create};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// стуктрура события
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    pub task_uuid: Uuid,
    pub task_name: String,
    pub duration: u64
}

fn handle_event_creation(path: String) {
    // начало обработки сообщения
    tokio::spawn(
            async move {
                info!("Начата обработка задачи из файла по пути {} ", &path);

                // получение сериализованной строки из файла
                let task_serialized = match fs::read_to_string(&path) {
                    Err(err) => {
                        error!("не удалось прочитать файл по пути: {}, ошибка: {}", path, err);
                        return;
                    }
                    Ok(task_serialized) => task_serialized
                };

                // десериализация строки в объект
                let task: Task  = match serde_json::from_str(&task_serialized) {
                    Err(err) => {
                        error!("ошибка десериализации объекта: {:?}, ошибка: {}", task_serialized, err);
                        return;
                    }
                    Ok(task) => task
                };

                // обработка
                tokio::time::sleep(Duration::new(task.duration, 0)).await;

                // удаление изначального файла
                if let Err(err) = fs::remove_file(&path) {
                    error!("не удалось удалить файл по пути: {}, ошибка: {}", path, err);
                    return;
                }

                // создание строки пути до файла
                let path = format!("../data/completed/{}.json", task.task_uuid);

                // запись в файловую систему
                if let Err(err) = fs::write(&path, &task_serialized) {
                    error!("ошибка записи результата задания: {:?} по пути {}, ошибка: {}", &task_serialized, path, err);
                    return;
                }

                info!("Задача с Uuid: {} завершена и записана в завершённые", task.task_uuid);
        }
    );
}

#[tokio::main]
async fn main() {
    // инициализация логирования
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // создание объекта отслеживания изменений и начало отслеживания
    let (tx, rx) = mpsc::channel::<Result<notify::Event>>();
    let mut watcher = recommended_watcher(tx).expect("Не удалось создать объект notify для отслеживания изменений в файлах");
    watcher.watch(Path::new("../data/tasks"), RecursiveMode::Recursive).expect("Не удалось запустить отслеживание новых ивентов");

    // создание папки для завершенных заданий
    fs::create_dir_all("../data/completed").expect("не удалось создать папку");

    info!("начало обработки задач");
    // обработка каждого изменения
    for res in rx {
        // обработка резултата отслеживания
        let mut event = match res {
            Err(err) => {
                error!("не удалось обработать результат Wathcher-а notify, ошибка: {}", err);
                continue;
            }
            Ok(event) => event
        };

        // если событие - создание файла: обработка
        if event.kind == Create(Any) {
            // получение пути добавленного файла
            let path = match event.paths.remove(0).into_os_string().into_string() {
                Err(os_string) => {
                    error!("не удалось зодать путь для файла, OsString: {:?}", os_string);
                    continue;
                }
                Ok(path) => path
            };

            // выполнение задачи и запись в законченные
            handle_event_creation(path)
        }
    }
}