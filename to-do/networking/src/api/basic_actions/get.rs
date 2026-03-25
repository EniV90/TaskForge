use actix_web::{HttpRequest, HttpResponse};
use to_do_core::api::basic_actions::get::{
    get_all as get_all_core, get_by_name as get_by_name_core,
};

use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};

pub async fn get_all() -> Result<HttpResponse, SchedulerServiceError> {
   
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}

pub async fn get_by_name(req: HttpRequest) -> Result<HttpResponse, SchedulerServiceError> {
    let name = match req.match_info().get("name") {
        Some(name) => name,
        None => {
            return Err(SchedulerServiceError::new(
                "Name not found".to_string(),
                SchedulerServiceErrorStatus::BadRequest,
            ));
        }
    };
    println!("Handler hit: {}", name);
    println!("Incoming path: {}", req.path());

    Ok(HttpResponse::Ok().json(get_by_name_core(name).await?))
}
