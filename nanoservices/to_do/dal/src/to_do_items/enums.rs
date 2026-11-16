use std::{fmt::Display, io::ErrorKind::InvalidInput};

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
impl TaskStatus {
    pub fn from_string(status: &String) -> Result<TaskStatus, std::io::Error> {
        match status.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::DONE),
            "PENDING" => Ok(TaskStatus::PENDING),
            _ => Err(std::io::Error::new(
                InvalidInput,
                format!("Invalid status: {}", status),
            )),
        }
    }
}
