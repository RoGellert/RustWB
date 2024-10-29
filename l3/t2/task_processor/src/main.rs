use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;
use tracing::{info, Level};
use notify::{recommended_watcher, RecursiveMode, Result, Watcher};
use notify::event::CreateKind::Any;
use notify::EventKind::{Create, Modify};
use serde::{Deserialize, Serialize};
use serde::de::Error;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    // стуктрура события
    pub task_uuid: Uuid,
    pub task_name: String,
    pub duration: u64
}

fn handle_event_creation(path: String) -> Result<()> {
    let task_serialized = fs::read_to_string(path)?;

    let task: Task  = serde_json::from_str(&task_serialized).unwrap();
    tokio::spawn(
            async move {
                tokio::time::sleep(Duration::new(task.duration, 0)).await;
            info!("aaaa");
        }
    );
    Ok(())
}

#[tokio::main]
async fn main() {
    // инициализация логирования
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let (tx, rx) = mpsc::channel::<Result<notify::Event>>();
    let mut watcher = recommended_watcher(tx).expect("Не удалось создать объект notify для отслеживания изменений в файлах");
    watcher.watch(Path::new("../data/tasks"), RecursiveMode::Recursive).expect("Не удалось запустить отслеживание новых ивентов");

    for res in rx {
        let mut event = res.expect("");
        if event.kind == Create(Any) {
            let path = event.paths.remove(0).into_os_string().into_string().unwrap();

            handle_event_creation(path).unwrap()
            // let task_serialized = fs::read_to_string(path).unwrap();
            // let task: Task  = serde_json::from_str(&task_serialized).unwrap();
            // tokio::spawn(
            //     async move {
            //         tokio::time::sleep(Duration::new(task.duration, 0)).await;
            //         info!("aaaa");
            //     }
            // );
        }
    }
}