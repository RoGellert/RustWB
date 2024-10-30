use notify::event::CreateKind::Any;
use notify::EventKind::Create;
use notify::{recommended_watcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;
use tokio::time::{sleep, Instant};
use tracing::{error, info, Level};
use uuid::Uuid;

// стуктрура задачи
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    pub task_uuid: Uuid,
    pub task_type: String, // тип задания
    pub n: usize,          // входной n
}

// стуктура выполненной задачи
#[derive(Debug, Serialize, Deserialize)]
struct CompletedTask {
    pub task_uuid: Uuid,
    pub task_type: String,
    pub n: usize,
    pub elapsed: u64, // время выполнения задачи в секундах
    pub result: f64,  // результат
}

impl CompletedTask {
    pub fn new(task: Task, elapsed: u64, result: f64) -> Self {
        CompletedTask {
            task_uuid: task.task_uuid,
            task_type: task.task_type,
            n: task.n,
            elapsed,
            result,
        }
    }
}

// функция которая выполняет задание в зависимости от типа задания и измеряет время выполнения
async fn process_task(task: Task) -> Result<CompletedTask, String> {
    // начало отсчета
    let start = Instant::now();

    // вычисление результата
    let result = match task.task_type.as_str() {
        "fibonnaci" => fibonacci(task.n),
        "tribonnaci" => tribonacci(task.n),
        "sum_of_square_roots" => sum_of_square_roots(task.n),
        "sum_of_squares" => sum_of_squares(task.n),
        "sleep" => {
            sleep(Duration::new(task.n as u64, 0)).await;
            0f64
        },
        _ => return Err(format!("неверный тип задачи: {}", task.task_type)),
    };

    // запись прошедшего времени
    let elapsed = start.elapsed().as_secs();

    // создание и возврат структуры завершенной задачи
    let completed_task = CompletedTask::new(task, elapsed, result);
    Ok(completed_task)
}

/// возможные задачи

// сумма квадратных корней от 1 до n
fn sum_of_square_roots(n: usize) -> f64 {
    let mut sum = 0.0;
    for i in 1..=n {
        sum += (i as f64).sqrt();
    }
    sum
}

// n-ный номер фибонначи
fn fibonacci(n: usize) -> f64 {
    // если n = 0 - возврат 0
    if n == 0 {
        return 0.0;
    }

    // подсчёт до n
    let (mut a, mut b) = (0, 1);
    for _ in 2..=n {
        let next: i64 = a + b;
        a = b;
        b = next;
    }
    // возврат как float
    b as f64
}

// n-ный номер триибонначи
fn tribonacci(n: usize) -> f64 {
    if n == 0 {
        // если n = 0 - возврат 0
        return 0.0;
    } else if n == 1 || n == 2 {
        // если n = 0 || n = 1 - возврат 0
        return 1.0;
    }

    // подсчёт до n
    let (mut a, mut b, mut c) = (0, 1, 1);
    for _ in 3..=n {
        let next = a + b + c;
        a = b;
        b = c;
        c = next;
    }
    // возврат как float
    c as f64
}

// сумма квадратов
fn sum_of_squares(n: usize) -> f64 {
    let mut sum: u64 = 0;
    for i  in 1..=n {
        let i = i as u64;
        sum += i * i;
    }
    sum as f64
}

fn handle_event_creation(path: String) {
    // обработка сообщения в отдельном трэде
    tokio::spawn(async move {
        info!("Начата обработка задачи из файла по пути {} ", &path);

        // получение сериализованной строки из файла
        let task_serialized = match fs::read_to_string(&path) {
            Err(err) => {
                error!(
                    "не удалось прочитать файл по пути: {}, ошибка: {}",
                    path, err
                );
                return;
            }
            Ok(task_serialized) => task_serialized,
        };

        // удаление изначального файла
        if let Err(err) = fs::remove_file(&path) {
            error!("не удалось удалить файл по пути: {}, ошибка: {}", path, err);
            return;
        }

        // десериализация строки в объект
        let task: Task = match serde_json::from_str(&task_serialized) {
            Err(err) => {
                error!(
                    "ошибка десериализации объекта: {:?}, ошибка: {}",
                    task_serialized, err
                );
                return;
            }
            Ok(task) => task,
        };

        // обработка
        let completed_task = match process_task(task).await {
            Err(err_str) => {
                error!(
                    "ошибка выполнения задания: {:?}, ошибка: {}",
                    task_serialized, err_str
                );
                return;
            }
            Ok(completed_task) => completed_task,
        };

        // сериализация
        let completed_task_serialised = match serde_json::to_string(&completed_task) {
            Err(err) => {
                error!(
                    "ошибка сериализации: {:?}, ошибка: {}",
                    task_serialized, err
                );
                return;
            }
            Ok(completed_task_serialised) => completed_task_serialised,
        };

        // создание строки пути до файла
        let path = format!("../data/completed/{}.json", completed_task.task_uuid);

        // запись в файловую систему
        if let Err(err) = fs::write(&path, &completed_task_serialised) {
            error!(
                "ошибка записи результата задания: {:?} по пути {}, ошибка: {}",
                &task_serialized, path, err
            );
            return;
        }

        info!(
            "Задача с Uuid: {} завершена и записана в завершённые, \
                тип задачи: {}, результат: {}, время выполнения: {}",
            completed_task.task_uuid,
            completed_task.task_type,
            completed_task.result,
            completed_task.elapsed
        );
    });
}

#[tokio::main]
async fn main() {
    // инициализация логирования
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // создание объекта отслеживания изменений и начало отслеживания
    let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();
    let mut watcher = recommended_watcher(tx)
        .expect("Не удалось создать объект notify для отслеживания изменений в файлах");
    watcher
        .watch(Path::new("../data/tasks"), RecursiveMode::Recursive)
        .expect("Не удалось запустить отслеживание новых ивентов");

    // создание папки для завершенных заданий
    fs::create_dir_all("../data/completed").expect("не удалось создать папку");

    info!("начало обработки задач");
    // обработка каждого изменения
    for res in rx {
        // обработка резултата отслеживания
        let mut event = match res {
            Err(err) => {
                error!(
                    "не удалось обработать результат Wathcher-а notify, ошибка: {}",
                    err
                );
                continue;
            }
            Ok(event) => event,
        };

        // если событие - создание файла: обработка
        if event.kind == Create(Any) {
            // получение пути добавленного файла
            let path = match event.paths.remove(0).into_os_string().into_string() {
                Err(os_string) => {
                    error!(
                        "не удалось создать путь для файла, OsString: {:?}",
                        os_string
                    );
                    continue;
                }
                Ok(path) => path,
            };

            // выполнение задачи и запись в законченные
            handle_event_creation(path)
        }
    }
}
