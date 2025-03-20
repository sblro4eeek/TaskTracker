use anyhow::Result;
use clap::Parser;
use prettytable::{Table, Row, Cell};

use crate::cli::Args;
use crate::task::{Task, TaskStatus, TaskFilter, Priority};
use crate::storage::{read_tasks, write_tasks, add_task};

mod cli;
mod task;
mod storage;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        cli::Commands::Add { description, priority } => {
            if !(1..=3).contains(&priority) {
                eprintln!("Приоритет должен быть в диапазоне 1..3");
                return Ok(());
            }
            let task = Task {
                id: 0,
                description,
                status: TaskStatus::InProgress,
                priority: Task::priority_from_u8(priority),
            };
            with_tasks(|tasks| {
                add_task(&task, tasks);
                Ok(())
            })?;
            println!("Задача успешно добавлена.");
        }
        cli::Commands::List { filter } => {
            let tasks = read_tasks()?;
            let filtered_tasks: Vec<&Task> = tasks.iter()
                .filter(|task| match filter {
                    TaskFilter::All => true,
                    TaskFilter::Done => matches!(task.status, TaskStatus::Done),
                    TaskFilter::InProgress => matches!(task.status, TaskStatus::InProgress),
                    TaskFilter::Cancelled => matches!(task.status, TaskStatus::Cancelled),
                    TaskFilter::Low => task.priority == Priority::Low,
                    TaskFilter::Medium => task.priority == Priority::Medium,
                    TaskFilter::High => task.priority == Priority::High,
                })
                .collect();

            if filtered_tasks.is_empty() {
                println!("Нет задач, соответствующих фильтру {:?}", filter);
            } else {
                let mut table = Table::new();
                table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
                table.set_titles(Row::new(vec![
                    Cell::new("ID"),
                    Cell::new("Описание"),
                    Cell::new("Статус"),
                    Cell::new("Приоритет"),
                ]));
                for task in filtered_tasks {
                    table.add_row(Row::new(vec![
                        Cell::new(&task.id.to_string()),
                        Cell::new(&task.description),
                        Cell::new(&task.status.to_string()),
                        Cell::new(&format!("{:?}", task.priority)),
                    ]));
                }
                println!("Список задач:");
                table.printstd();
            }
        }
        cli::Commands::Update { id, status, description } => {
            with_tasks(|tasks| {
                let task = tasks.iter_mut()
                    .find(|t| t.id == id as u32)
                    .ok_or_else(|| anyhow::anyhow!("Задача с ID {} не найдена", id))?;
                if let Some(new_status) = status {
                    task.status = new_status;
                }
                if let Some(new_description) = description {
                    task.description = new_description;
                }
                println!("Задача #{} обновлена.", id);
                Ok(())
            })?;
        }
        cli::Commands::Remove { id } => {
            with_tasks(|tasks| {
                let initial_len = tasks.len();
                tasks.retain(|task| task.id != id);
                if tasks.len() == initial_len {
                    anyhow::bail!("Задача с ID {} не найдена", id);
                }
                println!("Задача #{} успешно удалена.", id);
                Ok(())
            })?;
        }
    }
    Ok(())
}

fn with_tasks<F>(operation: F) -> Result<()>
where
    F: FnOnce(&mut Vec<Task>) -> Result<()>,
{
    let mut tasks = read_tasks()?;
    operation(&mut tasks)?;
    write_tasks(&tasks)?;
    Ok(())
}