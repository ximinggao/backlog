use crate::{enums::TaskStatus, structs::ToDoItem};

use glue::errors::NanoServiceError;

#[cfg(feature = "json-file-storage")]
use to_do_dal::json_file::save_one;

pub fn create(title: &str, status: TaskStatus) -> Result<ToDoItem, NanoServiceError> {
    let item = ToDoItem {
        title: title.to_string(),
        status: status,
    };
    let _ = save_one(&title.to_string(), &item)?;
    Ok(item)
}

pub async fn acreate(item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
    let _ = save_one(&item.title, &item)?;
    Ok(item)
}
