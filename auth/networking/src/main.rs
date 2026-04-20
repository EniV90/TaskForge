use actix_web::{App, HttpServer};
mod api;
use api::views_factory;
use auth_dal::migrations::run_migrations;
pub mod extract_auth;
use glue::logger::{logger::init_logger, network_wrapper::actix_web::ActixLogger};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    run_migrations().await;
    HttpServer::new(|| App::new().wrap(ActixLogger).configure(views_factory))
        .workers(4)
        .bind("127.0.0.1:8081")?
        .run()
        .await
}
