use anyhow::{Result, Context};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read};

use crate::task::Task;

pub fn read_tasks() -> Result<Vec<Task>> {
    let file_name = "tasks.json";
    match File::open(file_name) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            reader.read_to_string(&mut contents).context("Не удалось прочитать файл")?;
            if contents.trim().is_empty() {
                println!("Файл пустой, возвращаем пустой список");
                Ok(Vec::new())
            } else {
                let tasks: Vec<Task> = serde_json::from_str(&contents).context("Ошибка десериализации задач")?;
                Ok(tasks)
            }
        }
        Err(_) => {
            println!("Файл не найден, возвращаем пустой список");
            Ok(Vec::new())
        }
    }
}

pub fn write_tasks(tasks: &[Task]) -> Result<()> {
    let file_name = "tasks.json";
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .context("Не удалось открыть файл для записи")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks).context("Ошибка сериализации задач")?;
    Ok(())
}

pub fn add_task(task: &Task, tasks: &mut Vec<Task>) {
    let max_id = tasks.iter().map(|t| t.id).max().unwrap_or(0);
    let new_task = Task {
        id: max_id + 1,
        ..task.clone()
    };
    println!("Добавлена задача: {:?}", &new_task);
    tasks.push(new_task);
}