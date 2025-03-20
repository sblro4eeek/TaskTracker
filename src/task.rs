use clap::ValueEnum;
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TaskStatus,
    pub priority: Priority,
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize, PartialEq)]
pub enum TaskFilter {
    All,
    Done,
    InProgress,
    Cancelled,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Done,
    InProgress,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Done => write!(f, "Done"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

impl Task {
    pub fn priority_from_u8(priority: u8) -> Priority {
        match priority {
            1 => Priority::Low,
            2 => Priority::Medium,
            3 => Priority::High,
            _ => Priority::Medium,
        }
    }
}