use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use rust_embed::RustEmbed;
use std::path::Path;

use actix_cors::Cors;
use to_do_server::api::views_factory as to_do_views_factory;

// Embedding Html
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../index.html"))
}

// Embedding React
#[derive(RustEmbed)]
#[folder = "../frontend/public"]
struct FrontendAssests;

// Serve the embedded frontend
fn serve_frontend_assest(path: String) -> HttpResponse {
    let file = match Path::new(&path).file_name() {
        Some(file) => file.to_str().unwrap(),
        None => return HttpResponse::BadRequest().body("404 Not Found"),
    };
    match FrontendAssests::get(file) {
        Some(content) => HttpResponse::Ok()
            .content_type(
                mime_guess::from_path(&file)
                    .first_or_octet_stream()
                    .as_ref(),
            )
            .append_header(("Cache-Control", "public, max-age=604800"))
            .body(content.data),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

async fn catch_all(req: HttpRequest) -> impl Responder {
    // validation to check if the request is for a backendend point
    // if it has a /api/ then it returns a not found
    if req.path().contains("/api/") {
        return HttpResponse::NotFound().finish();
    }
    // validation to serve frontend assests if there is /frontend/public path folder
    if req.path().contains("frontend/public") {
        return serve_frontend_assest(req.path().to_string());
    }
    // validation check to inspect the file type
    let file_type = match mime_guess::from_path(&req.path()).first_raw() {
        Some(file_type) => file_type,
        None => "text/html",
    };

    if !file_type.contains("text/html") {
        return serve_frontend_assest(req.path().to_string());
    }
    index().await
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .configure(to_do_views_factory)
            .wrap(cors)
            .default_service(web::route().to(catch_all))
    })
    .bind("0.0.0.0:8001")?
    .run()
    .await
}
