pub mod api;
use actix_web::{App, HttpServer};
use api::views_factory;
use auth_dal::migrations::run_migrations;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run_migrations().await;
    HttpServer::new(|| App::new().configure(views_factory))
        .workers(4)
        .bind("127.0.0.1:8081")?
        .run()
        .await
}
