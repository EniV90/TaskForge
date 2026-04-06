use crate::to_do_items::schema::ToDoItem;
use glue::errors::SchedulerServiceError;
use std::future::Future;

#[cfg(feature = "json-file")]
use super::super::descriptors::JsonFileDescriptor;
#[cfg(feature = "json-file")]
use crate::json_file::{get_all, save_all};
#[cfg(feature = "json-file")]
use std::collections::HashMap;

#[cfg(feature = "sqlx-postgres")]
use super::super::descriptors::SqlxPostGresDescriptor;
#[cfg(feature = "sqlx-postgres")]
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
#[cfg(any(feature = "sqlx-postgres", feature = "json-file"))]
use glue::errors::SchedulerServiceErrorStatus;

pub trait DeleteOne {
    fn delete_one(
        title: String,
    ) -> impl Future<Output = Result<ToDoItem, SchedulerServiceError>> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl DeleteOne for SqlxPostGresDescriptor {
    fn delete_one(
        title: String,
    ) -> impl Future<Output = Result<ToDoItem, SchedulerServiceError>> + Send {
        sqlx_postgres_delete_one(title)
    }
}

#[cfg(feature = "json-file")]
impl DeleteOne for JsonFileDescriptor {
    fn delete_one(
        title: String,
    ) -> impl Future<Output = Result<ToDoItem, SchedulerServiceError>> + Send {
        json_file_delete_one(title)
    }
}

#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_delete_one(title: String) -> Result<ToDoItem, SchedulerServiceError> {
    let item =
        sqlx::query_as::<_, ToDoItem>(" DELETE FROM to_do_items WHERE title = $1 RETURNING *")
            .bind(title)
            .fetch_one(&*SQLX_POSTGRES_POOL)
            .await
            .map_err(|e| {
                SchedulerServiceError::new(e.to_string(), SchedulerServiceErrorStatus::Unknown)
            })?;
    Ok(item)
}

#[cfg(feature = "json-file")]
async fn json_file_delete_one(title: String) -> Result<ToDoItem, SchedulerServiceError> {
    let mut tasks = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
    let to_do_item = tasks.remove(&title).ok_or_else(|| {
        SchedulerServiceError::new(
            "Item not found".to_string(),
            SchedulerServiceErrorStatus::NotFound,
        )
    })?;
    let _ = save_all(&tasks)?;
    Ok(to_do_item)
}
