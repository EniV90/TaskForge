use actix_web::{HttpResponse, web::Json};
use auth_kernel::api::users::get::get_user_by_unique_id;
use glue::errors::SchedulerServiceError;
use glue::token::HeaderToken;
use to_do_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};

use auth_kernel::user_sessions::transactions::get::GetUserSession;
use to_do_dal::to_do_items::schema::NewToDoItem;
use to_do_dal::to_do_items::transactions::{create::SaveOne, get::GetAll};

pub async fn create<T, X>(
    token: HeaderToken,
    body: Json<NewToDoItem>,
) -> Result<HttpResponse, SchedulerServiceError>
where
    T: SaveOne + GetAll,
    X: GetUserSession,
{
    let session = X::get_user_session(token.unique_id).await?;
    let _ = create_core::<T>(body.into_inner(), session.user_id).await?;

    Ok(HttpResponse::Created().json(get_all_core::<T>(session.user_id).await?))
}
