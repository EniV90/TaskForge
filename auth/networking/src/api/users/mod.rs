pub mod create;
pub mod get;

use actix_web::web::{self, ServiceConfig};
use auth_dal::users::descriptors::SqlxPostGresDescriptor;

pub fn users_factory(app: &mut ServiceConfig) {
    app.service(
        web::scope("/api/v1/users")
            .service(
                web::resource("create")
                    .route(web::post().to(create::create::<SqlxPostGresDescriptor>)),
            )
            .service(
                web::resource("get")
                    .route(web::get().to(get::get_by_unique_id::<SqlxPostGresDescriptor>)),
            ),
    );
}
