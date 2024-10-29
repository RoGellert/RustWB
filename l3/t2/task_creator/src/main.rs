use std::fs;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use serde::{Deserialize, Serialize};
use tracing::{error, info, Level};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct TaskPayload {
    pub task_name: String,
    pub duration: u32
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    pub task_uuid: Uuid,
    pub task_name: String,
    pub duration: u32
}

impl Task {
    pub fn from_payload(task_payload: TaskPayload) -> Self {
        let task_uuid = Uuid::new_v4();
        Task {
            task_uuid,
            task_name: task_payload.task_name,
            duration: task_payload.duration
        }
    }
}

// потенциальные ошибки
#[derive(Debug)]
pub enum ServerError {
    Serialisation(String),
    FileSystem(String)
}


// для обработки потенциальных ошибок сервером Axum
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::FileSystem(err) => {
                error!("Ошибка файловой системы {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ошибка файловой системы {:?}", err),
                )
                    .into_response()
            },
            ServerError::Serialisation(err) => {
                error!("Ошибка сериализации {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Ошибка сериализации {:?}", err),
                )
                    .into_response()
            }
        }
    }
}

// создание новой задачи при запросе в веб-сервер
async fn create_task(Json(task_payload): Json<TaskPayload>) -> Result<(), ServerError> {
    // преобразование содержимого Body в Task
    let new_task = Task::from_payload(task_payload);

    // создание строки пути до файла
    let path = format!("../data/tasks/{}.json", new_task.task_uuid);

    // сериализация данных
    let data = serde_json::to_string(&new_task).map_err(|err| ServerError::Serialisation(format!("Не удалось сериализовать объект: {:?}, ошибка: {} ", &new_task, err)))?;

    // запись в фаловую систему
    fs::write(path, data).map_err(|err| ServerError::FileSystem(format!("Не удалось записать файл в файловую систему: {:?}, ошибка: {}", &new_task, err)))?;

    info!("Создана новая задача с Uuid: {}", new_task.task_uuid);

    Ok(())
}

#[tokio::main]
async fn main() {
    // инициализация логирования
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // создание папки для заданий
    fs::create_dir_all("../data/tasks").expect("не удалось создать папку");

    // конфигурация энд-поинта
    let app = Router::new()
        .route("/tasks", post(create_task));

    // старт сервера на порту 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("не удалось создать TcpListener");
    info!("Сервер AXUM готов принимать запросы на порту 3000");
    axum::serve(listener, app)
        .await
        .expect("не удалось запустить сервер AXUM");
}
