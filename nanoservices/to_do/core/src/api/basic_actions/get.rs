use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::{schema::AllToDoItems, transactions::get::GetAll};

pub async fn get_all<T: GetAll>() -> Result<AllToDoItems, NanoServiceError> {
    Ok(T::get_all().await?)
}
