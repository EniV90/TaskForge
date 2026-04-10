#[cfg(any(feature = "auth-core", feature = "http"))]
mod common_imports {
    pub use auth_dal::users::schema::TrimmedUser;
    pub use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
}

#[cfg(feature = "auth-core")]
mod core_imports {
    pub use auth_core::api::users::get::get_by_unique_id as get_by_unique_id_core;
    pub use auth_dal::users::descriptors::SqlxPostGresDescriptor;
}

#[cfg(feature = "http")]
mod reqwest_imports {
    pub use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
    pub use glue::token::HeaderToken;
    pub use reqwest::Client;
}

use auth_dal::users::schema::TrimmedUser;
#[cfg(any(feature = "core-postgres", feature = "http"))]
use common_imports::*;
#[cfg(feature = "core-postgres")]
use core_imports::*;
use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
#[cfg(feature = "http")]
use reqwest_imports::*;

#[cfg(any(feature = "core-postgres", feature = "http"))]
pub async fn get_user_by_unique_id(id: String) -> Result<TrimmedUser, SchedulerServiceError> {
    #[cfg(feature = "core-postgres")]
    let user: TrimmedUser = get_by_unique_id_core::<SqlxPostGresDescriptor>(id)
        .await?
        .into();
    #[cfg(feature = "http")]
    let user: TrimmedUser = get_user_by_unique_id_api_call(id).await?.into();
    return Ok(user);
}

#[cfg(feature = "http")]
pub async fn get_user_by_unique_id_api_call(
    id: String,
) -> Result<TrimmedUser, SchedulerServiceError> {
    let url = std::env::var("AUTH_API_URL").map_err(|e| {
        SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::BadRequest)
    })?;
    let full_url = format!("{}/api/v1/users/get", url);

    let header_token = HeaderToken { unique_id: id }.encode()?;
    let client = Client::new();
    let response = client
        .get(&full_url)
        .header("token", header_token)
        .send()
        .await
        .map_err(|e| {
            SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::BadRequest)
        })?;

    if response.status().is_success() {
        let trimmed_user = response.json::<TrimmedUser>().await.map_err(|e| {
            SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::BadRequest)
        })?;
        return Ok(trimmed_user);
    } else {
        return Err(SchedulerServiceError::new(
            format!("Failed to get user: {}", response.status()),
            SchedulerServiceErrorStatus::BadRequest,
        ));
    }
}
