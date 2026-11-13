use crate::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;
use std::future::Future;

#[cfg(feature = "json-file")]
use crate::json_file::get_all;
#[cfg(feature = "json-file")]
use crate::to_do_items::descriptors::JsonFileDescriptor;
#[cfg(feature = "json-file")]
use std::collections::HashMap;

#[cfg(feature = "sqlx-postgres")]
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
#[cfg(feature = "sqlx-postgres")]
use crate::to_do_items::descriptors::SqlxPostgresDescriptor;
#[cfg(any(feature = "sqlx-postgres", feature = "json-file"))]
use glue::errors::NanoServiceErrorStatus;

pub trait UpdateOne {
    fn update_one(
        item: ToDoItem,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl UpdateOne for SqlxPostgresDescriptor {
    fn update_one(
        item: ToDoItem,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        sqlx_postgres_update_one(item)
    }
}

#[cfg(feature = "json-file")]
impl UpdateOne for JsonFileDescriptor {
    fn update_one(
        item: ToDoItem,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        json_file_update_one(item)
    }
}

#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_update_one(item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
    let updated_item = sqlx::query_as::<_, ToDoItem>(
        "UPDATE to_do_items SET title = $1, status = $2 WHERE id = $3 RETURNING *",
    )
    .bind(item.title)
    .bind(item.status.to_string())
    .bind(item.id)
    .fetch_one(&*SQLX_POSTGRES_POOL)
    .await
    .map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown))?;
    Ok(updated_item)
}

#[cfg(feature = "json-file")]
async fn json_file_update_one(item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
    let mut tasks = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
    if tasks.contains_key(&item.title) {
        tasks.insert(item.title.clone(), item.clone());
        // Here you would typically save the updated tasks back to the JSON file
        // For example: save_all(&tasks).map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown))?;
        Ok(item)
    } else {
        Err(NanoServiceError::new(
            "Item not found".to_string(),
            NanoServiceErrorStatus::NotFound,
        ))
    }
}
