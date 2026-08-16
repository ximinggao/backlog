use actix_web::{HttpResponse, web::Json};
use glue::errors::NanoServiceError;
use to_do_core::{
    api::basic_actions::{create::acreate, get::get_all},
    structs::ToDoItem,
};

pub async fn create(body: Json<ToDoItem>) -> Result<HttpResponse, NanoServiceError> {
    let _ = acreate(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all().await?))
}
