pub mod login;
pub mod logout;

use actix_web::web::{self, ServiceConfig};
use auth_dal::users::descriptors::SqlxPostGresDescriptor;
pub fn auth_factory(app: &mut ServiceConfig) {
    app.service(web::scope("api/v1/auth").service(
        web::resource("/login").route(web::get().to(login::login::<SqlxPostGresDescriptor>)),
    ));
}
