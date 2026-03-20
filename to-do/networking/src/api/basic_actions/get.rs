use actix_web::HttpResponse;
use to_do_core::api::basic_actions::get::get_all as get_all_core;

use glue::errors::SchedulerServiceError;

pub async fn get_all() -> Result<HttpResponse, SchedulerServiceError> {
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
