use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::{schema::ToDoItem, transactions::get::GetAll};

pub async fn get_all<T: GetAll>() -> Result<Vec<ToDoItem>, NanoServiceError> {
    Ok(T::get_all().await?)
}
