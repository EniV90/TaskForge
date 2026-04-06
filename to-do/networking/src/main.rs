mod api;
use to_do_dal::migrations::run_migrations;

use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run_migrations().await;
    HttpServer::new(|| App::new().configure(api::views_factory))
        .workers(4)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
