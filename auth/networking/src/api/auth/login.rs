use crate::extract_auth::extrect_credentials;
use actix_web::HttpResponse;
use auth_core::api::auth::login::login as core_login;
use auth_dal::users::transactions::get::GetByEmail;
use glue::errors::SchedulerServiceError;

pub async fn login<T: GetByEmail>(
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, SchedulerServiceError> {
    let credentials = extrect_credentials(req).await?;
    let token = core_login::<T>(credentials.email, credentials.password).await?;
    Ok(HttpResponse::Ok().json(token))
}
