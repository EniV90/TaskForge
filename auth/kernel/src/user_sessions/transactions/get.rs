use crate::user_sessions::descriptors::RedisSessionDescriptor;
use crate::user_sessions::schema::UserSession;
use crate::{api::users::get::get_user_by_unique_id, user_sessions};
use cache_client::{UserSessionStatus, login, update};
use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
use std::{future::Future, process::Output};

pub trait GetUserSession {
    fn get_user_session(
        unique_id: String,
    ) -> impl Future<Output = Result<UserSession, SchedulerServiceError>>;
}

impl GetUserSession for RedisSessionDescriptor {
    fn get_user_session(
        unique_id: String,
    ) -> impl Future<Output = Result<UserSession, SchedulerServiceError>> {
        get_session_redis(unique_id)
    }
}

pub async fn get_session_redis(unique_id: String) -> Result<UserSession, SchedulerServiceError> {
    let address = std::env::var("CACHE_API_ERR").map_err(|e| {
        SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::BadRequest)
    })?;
    let user_id = update(&address, &unique_id).await?;
    match user_id {
        UserSessionStatus::Ok(id) => Ok(UserSession { user_id: id }),
        UserSessionStatus::Refresh => {
            let user = get_user_by_unique_id(unique_id.clone()).await?;
            let _ = login(&address, &unique_id, 20, user.id).await?;
            match user_id {
                UserSessionStatus::Ok(id) => Ok(UserSession { user_id: id }),
                _ => Err(SchedulerServiceError::new(
                    "Failed to update user session".to_string(),
                    SchedulerServiceErrorStatus::Unknown,
                )),
            }
        }
    }
}
