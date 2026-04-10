use glue::errors::SchedulerServiceError;
use to_do_dal::to_do_items::transactions::delete::DeleteOne;

pub async fn delete<T: DeleteOne>(id: &str, user_id: i32) -> Result<(), SchedulerServiceError> {
    let _ = T::delete_one(id.to_string(), user_id).await?;
    Ok(())
}
