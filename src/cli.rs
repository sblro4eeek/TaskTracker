use clap::{Parser, Subcommand};
use crate::task::{TaskFilter, TaskStatus};

#[derive(Parser, Debug)]
#[command(name = "task_tracker")]
#[command(about = "Утилита для управления задачами", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Добавить новую задачу
    Add {
        /// Описание задачи
        description: String,

        /// Приоритет задачи (1 - низкий, 2 - средний, 3 - высокий)
        #[arg(long, default_value_t = 2)]
        priority: u8,
    },

    /// Показать список задач
    List {
        /// Фильтр для списка (all, done, in-progress, cancelled, low, medium, high)
        #[arg(long, default_value = "all")]
        filter: TaskFilter,
    },

    /// Обновить задачу
    Update {
        /// ID задачи
        id: u8,

        /// Новый статус задачи
        #[arg(long)]
        status: Option<TaskStatus>,

        /// Новое описание задачи
        #[arg(long)]
        description: Option<String>,
    },

    /// Удалить задачу
    Remove {
        /// ID задачи для удаления
        id: u32,
    },
}