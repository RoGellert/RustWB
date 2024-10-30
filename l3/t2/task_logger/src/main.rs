use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc;
use tracing::{error, info, Level};
use notify::{recommended_watcher, RecursiveMode, Result, Watcher};
use notify::event::CreateKind::Any;
use notify::EventKind::{Create};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// стуктрура события
#[derive(Debug, Serialize, Deserialize)]
struct CompletedTask {
    pub task_uuid: Uuid,
    pub task_type: String,
    pub n: usize,
    pub elapsed: u64, // время выполнения задачи в секундах
    pub result: f64,  // результат
}

// структура лога
#[derive(Debug, Serialize)]
struct LogStruct {
    pub log_line: String,
    pub task: CompletedTask,
}

impl LogStruct {
    pub fn new(log_line: String, task: CompletedTask) -> Self {
        LogStruct {
            log_line,
            task
        }
    }
}

fn handle_event_completion(path: String, mut file: &File) {
    // получение сериализованной строки из файла
    let task_serialized = match fs::read_to_string(&path) {
        Err(err) => {
            error!("не удалось прочитать файл по пути: {}, ошибка: {}", path, err);
            return;
        }
        Ok(task_serialized) => task_serialized
    };

    // удаление изначального файла
    if let Err(err) = fs::remove_file(&path) {
        error!("не удалось удалить файл по пути: {}, ошибка: {}", path, err);
        return;
    }

    // десериализация строки в структуру
    let task: CompletedTask = match serde_json::from_str(&task_serialized) {
        Err(err) => {
            error!("ошибка десериализации объекта: {:?}, ошибка: {}", task_serialized, err);
            return;
        }
        Ok(task) => task
    };

    // создание структуры лога
    let log_struct = LogStruct::new(format!("Задача с uuid: {} завершена", task.task_uuid), task);

    // десериализация структуры в строку
    let log_struct_serialised  = match serde_json::to_string(&log_struct) {
        Err(err) => {
            error!("ошибка сериализации объекта: {:?}, ошибка: {}", task_serialized, err);
            return;
        }
        Ok(task) => task
    };

    // запись в файл лога
    if let Err(err) = writeln!(file, "{}", log_struct_serialised) {
        error!("ошибка записи лога: {} в файл: {:?}, ошибка: {}", log_struct_serialised, file, err);
        return
    }

    info!("{}", log_struct.log_line);
}

fn main() {
    // инициализация логирования
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // создание объекта отслеживания изменений и начало отслеживания
    let (tx, rx) = mpsc::channel::<Result<notify::Event>>();
    let mut watcher = recommended_watcher(tx).expect("Не удалось создать объект notify для отслеживания изменений в файлах");
    watcher.watch(Path::new("../data/completed"), RecursiveMode::Recursive).expect("Не удалось запустить отслеживание новых ивентов");

    // откытие файла для записи + создание его если он отсутствует
    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("../data/log.json").expect("не удалось создать файл для логов");

    info!("начало создания логов обработанных задач");

    // обработка каждого изменения
    for res in rx {
        // обработка результата отслеживания
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
                    error!("не удалось создать путь для файла, OsString: {:?}", os_string);
                    continue;
                }
                Ok(path) => path
            };

            // запись в лог
            handle_event_completion(path, &file)
        }
    }
}
