use actix_web::web::{ServiceConfig, delete, get, patch, post, scope};
use to_do_dal::to_do_items::descriptors::SqlxPostgresDescriptor;

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
            .route("get/all", get().to(get::get_all::<SqlxPostgresDescriptor>))
            // .route("get/{name}", get().to(get::get_by_name))
            .route(
                "create",
                post().to(create::create::<SqlxPostgresDescriptor>),
            )
            .route(
                "delete/{name}",
                delete().to(delete::delete_by_name::<SqlxPostgresDescriptor>),
            )
            .route(
                "update",
                patch().to(update::update::<SqlxPostgresDescriptor>),
            ),
    );
}
