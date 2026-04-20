mod api;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use glue::logger::{logger::init_logger, network_wrapper::actix_web::ActixLogger};
use to_do_dal::migrations::run_migrations;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    run_migrations().await;
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(ActixLogger)
            .wrap(cors)
            .configure(api::views_factory)
    })
    .workers(4)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
