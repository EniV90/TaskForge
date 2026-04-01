use actix_web::{HttpResponse, web::Json};
use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
use to_do_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};
use to_do_core::structs::TodoItems;

pub async fn create(body: Json<TodoItems>) -> Result<HttpResponse, SchedulerServiceError> {
    let _ = create_core(body.into_inner()).await?;

    Ok(HttpResponse::Created().json(get_all_core().await?))
}
