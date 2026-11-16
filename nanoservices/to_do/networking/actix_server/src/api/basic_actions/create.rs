use actix_web::{HttpResponse, web::Json};
use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::create::create as create_core;
use to_do_dal::to_do_items::{
    schema::NewToDoItem,
    transactions::create::SaveOne,
};

pub async fn create<T: SaveOne>(
    _token: HeaderToken,
    body: Json<NewToDoItem>,
) -> Result<HttpResponse, NanoServiceError> {
    let item = create_core::<T>(body.into_inner()).await?;
    Ok(HttpResponse::Created().json(item))
}
