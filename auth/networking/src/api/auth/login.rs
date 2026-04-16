use std::env::var;

use crate::extract_auth::extrect_credentials;
use actix_web::{HttpResponse};
use auth_core::api::auth::login::login as core_login;
use auth_dal::users::transactions::get::GetByEmail;
use auth_kernel::user_sessions::transactions::login::LoginUserSession;
use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};

pub async fn login<T, X>(req: actix_web::HttpRequest) -> Result<HttpResponse, SchedulerServiceError>
where
    T: GetByEmail,
    X: LoginUserSession,
{
    let credentials = extrect_credentials(req).await?;
    let token = core_login::<T>(credentials.email.clone(), credentials.password).await?;
    let user = T::get_by_email(credentials.email).await?;

    let url = std::env::var("CACHE_API_URL").map_err(|e| {
        SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::Unknown)
    })?;

    let _ = X::login_user_session(&url, &user.unique_id, 20, user.id).await?;

    Ok(HttpResponse::Ok().json(token))
}
