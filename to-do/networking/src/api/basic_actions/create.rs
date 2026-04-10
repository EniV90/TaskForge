use actix_web::{HttpResponse, web::Json};
use auth_kernel::api::users::get::get_user_by_unique_id;
use glue::errors::SchedulerServiceError;
use glue::token::HeaderToken;
use to_do_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};

use to_do_dal::to_do_items::schema::NewToDoItem;
use to_do_dal::to_do_items::transactions::{create::SaveOne, get::GetAll};

pub async fn create<T: SaveOne + GetAll>(
    token: HeaderToken,
    body: Json<NewToDoItem>,
) -> Result<HttpResponse, SchedulerServiceError> {
    let user = get_user_by_unique_id(token.unique_id).await?;
    let _ = create_core::<T>(body.into_inner(), user.id).await?;

    Ok(HttpResponse::Created().json(get_all_core::<T>(user.id).await?))
}
