use actix_web::{HttpResponse, web::Json};
use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
use glue::token::HeaderToken;
use to_do_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};
use to_do_core::structs::TodoItems;
use to_do_dal::to_do_items::schema::NewToDoItem;
use to_do_dal::to_do_items::transactions::{create::SaveOne, get::GetAll};

pub async fn create<T: SaveOne + GetAll>(
    token: HeaderToken,
    body: Json<NewToDoItem>,
) -> Result<HttpResponse, SchedulerServiceError> {
    let _ = create_core::<T>(body.into_inner()).await?;

    Ok(HttpResponse::Created().json(get_all_core::<T>().await?))
}
