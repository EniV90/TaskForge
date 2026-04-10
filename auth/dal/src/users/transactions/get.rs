use super::super::descriptors::SqlxPostGresDescriptor;
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
use crate::users::schema::User;
use glue::errors::{SchedulerServiceError, SchedulerServiceErrorStatus};
use std::future::Future;

pub trait GetByEmail {
    fn get_by_email(
        email: String,
    ) -> impl Future<Output = Result<User, SchedulerServiceError>> + Send;
}

impl GetByEmail for SqlxPostGresDescriptor {
    fn get_by_email(
        email: String,
    ) -> impl Future<Output = Result<User, SchedulerServiceError>> + Send {
        sqlx_postgres_get_by_email(email)
    }
}

async fn sqlx_postgres_get_by_email(email: String) -> Result<User, SchedulerServiceError> {
    let item = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| {
            SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::Unknown)
        })?;
    match item {
        None => Err(SchedulerServiceError::new(
            "User not found".to_string(),
            SchedulerServiceErrorStatus::NotFound,
        )),
        Some(item) => Ok(item),
    }
}

pub trait GetByUniqueId {
    fn get_by_unique_id(
        id: String,
    ) -> impl Future<Output = Result<User, SchedulerServiceError>> + Send;
}

impl GetByUniqueId for SqlxPostGresDescriptor {
    fn get_by_unique_id(
        id: String,
    ) -> impl Future<Output = Result<User, SchedulerServiceError>> + Send {
        sqlx_postgres_get_by_unique_id(id)
    }
}

async fn sqlx_postgres_get_by_unique_id(id: String) -> Result<User, SchedulerServiceError> {
    let item = sqlx::query_as::<_, User>(" SELECT * FROM users WHERE unique_id = $1")
        .bind(id)
        .fetch_optional(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| {
            SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::Unknown)
        })?;
    match item {
        None => Err(SchedulerServiceError::new(
            "User not found".to_string(),
            SchedulerServiceErrorStatus::NotFound,
        )),
        Some(item) => Ok(item),
    }
}
