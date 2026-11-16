use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::enums::TaskStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewToDoItem {
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx-postgres", derive(sqlx::FromRow))]
pub struct ToDoItem {
    pub id: i32,
    pub title: String,
    pub status: String,
}

impl Display for ToDoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ToDoItem {{ id: {}, title: {}, status: {} }}",
            self.id, self.title, self.status
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllToDoItems {
    pub pending: Vec<ToDoItem>,
    pub done: Vec<ToDoItem>,
}

impl AllToDoItems {
    pub fn from_vec(all_items: Vec<ToDoItem>) -> Self {
        let mut pending = Vec::new();
        let mut done = Vec::new();

        for item in all_items {
            match item.status.as_str() {
                "PENDING" => pending.push(item),
                "DONE" => done.push(item),
                _ => {}
            }
        }

        AllToDoItems { pending, done }
    }
}
