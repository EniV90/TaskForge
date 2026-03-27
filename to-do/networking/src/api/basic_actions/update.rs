use actix_web::{HttpResponse, web::Json};
use glue::errors::SchedulerServiceError;
use to_do_core::api::basic_actions::{get::get_all as get_all_core, update::update as update_core};
use to_do_core::structs::TodoItems;

pub async fn update(body: Json<TodoItems>) -> Result<HttpResponse, SchedulerServiceError> {
    let _ = update_core(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
