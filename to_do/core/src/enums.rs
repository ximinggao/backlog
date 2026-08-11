use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TaskStatus {
    PENDING,
    DONE,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::PENDING => write!(f, "PENDING"),
            TaskStatus::DONE => write!(f, "DONE"),
        }
    }
}
