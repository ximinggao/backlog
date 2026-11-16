use crate::to_do_items::schema::{AllToDoItems, ToDoItem};
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
#[cfg(feature = "sqlx-postgres")]
use glue::errors::NanoServiceErrorStatus;

pub trait GetAll {
    fn get_all() -> impl Future<Output = Result<AllToDoItems, NanoServiceError>> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl GetAll for SqlxPostgresDescriptor {
    fn get_all() -> impl Future<Output = Result<AllToDoItems, NanoServiceError>> + Send {
        sqlx_postgres_get_all()
    }
}

#[cfg(feature = "json-file")]
impl GetAll for JsonFileDescriptor {
    fn get_all() -> impl Future<Output = Result<AllToDoItems, NanoServiceError>> + Send {
        json_file_get_all()
    }
}

#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_get_all() -> Result<AllToDoItems, NanoServiceError> {
    let items = sqlx::query_as::<_, ToDoItem>("SELECT * FROM to_do_items")
        .fetch_all(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown))?;
    Ok(AllToDoItems::from_vec(items))
}

#[cfg(feature = "json-file")]
async fn json_file_get_all() -> Result<AllToDoItems, NanoServiceError> {
    let tasks = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
    Ok(AllToDoItems::from_vec(tasks.values().cloned().collect()))
}
