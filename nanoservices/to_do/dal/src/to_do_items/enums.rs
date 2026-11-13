use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    PENDING,
    DONE,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::PENDING => write!(f, "PENDING"),
            TaskStatus::DONE => write!(f, "DONE"),
        }
    }
}
