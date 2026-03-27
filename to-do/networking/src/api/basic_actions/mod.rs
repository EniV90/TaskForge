pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use actix_web::web::{self, ServiceConfig};

pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.service(
        web::scope("/api/v1")
            .service(web::resource("/get/all").route(web::get().to(get::get_all)))
            .service(web::resource("/get/{name}").route(web::get().to(get::get_by_name)))
            .service(web::resource("/create").route(web::post().to(create::create)))
            .service(
                web::resource("/delete/{name}").route(web::delete().to(delete::delete_by_name)),
            )
            .service(web::resource("/update").route(web::patch().to(update::update))),
    );
}
