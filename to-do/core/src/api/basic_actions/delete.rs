use glue::errors::SchedulerServiceError;
use to_do_dal::to_do_items::transactions::delete::DeleteOne;

pub async fn delete<T: DeleteOne>(id: &str) -> Result<(), SchedulerServiceError> {
    let _ = T::delete_one(id.to_string()).await?;
    Ok(())
}
