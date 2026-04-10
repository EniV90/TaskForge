use actix_web::{HttpRequest, HttpResponse};
use auth_kernel::api::users::get::get_user_by_unique_id;
use to_do_core::api::basic_actions::get::{
    get_all as get_all_core, get_by_name as get_by_name_core,
};
use to_do_dal::to_do_items::transactions::get::GetAll;

use glue::{
    errors::{SchedulerServiceError, SchedulerServiceErrorStatus},
    token::HeaderToken,
};

pub async fn get_all<T: GetAll>(token: HeaderToken) -> Result<HttpResponse, SchedulerServiceError> {
    let user = get_user_by_unique_id(token.unique_id).await?;
    Ok(HttpResponse::Ok().json(get_all_core::<T>(user.id).await?))
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
